/* TODO:
* Maybe some way to grab what we need to automatically @implements and @extends?
* Add options for Properties, Signals, Callbacks?
*/

#[macro_export(local_inner_macros)]
macro_rules! call_func {
    ($obj:ident, $func:ident) => {
        $obj.$func()
    };
}

// Problems with this system is that it assumes you have at least ONE widget you want to extend.
// Doesn't work if you just want an object without an extension of another object.
#[macro_export]
macro_rules! full_object {
    ($name:ident, $widget_type:ty, ($($extends:ty),*), ($($implements:ty),*), $(#[$metas:meta])+ struct $struct_def:tt, {$($subclass_stmt:tt)*}) => {
        use gtk::{subclass::prelude::*, glib, prelude::*};

        mod imp {
            use super::*;

            $(#[$metas])+
            pub struct $name $struct_def

            #[glib::object_subclass]
            impl ObjectSubclass for $name {
                const NAME : &'static str = concat!("JCC", stringify!($name));
                type Type = super::$name;
                type ParentType = $widget_type;
                
                $($subclass_stmt)*
            }
        }

        glib::wrapper! {
            pub struct $name(ObjectSubclass<imp::$name>) @extends $widget_type, $($extends,)* @implements $($implements),*;
        }
    };
}

#[macro_export]
macro_rules! quick_object {
    ($name:ident, $widget_type:ty, ($($extends:ty),*), ($($implements:ty),*), $(#[$metas:meta])+ struct $struct_def:tt) => {
        $crate::full_object!($name, $widget_type, ($($extends),*), ($($implements),*), $(#[$metas])+ struct $struct_def, {});
    };
}

// Hidden because there's nothing this does that full_object! can't do.
#[macro_export(local_inner_macros)]
macro_rules! full_template {
    ($name:ident, $widget_type:ty, ($($extends:ty),*), ($($implements:ty),*), $(#[$metas:meta])+ struct $struct_def:tt, ($($instance_callbacks:ident)?)) => {
        use gtk::CompositeTemplate;
        $crate::full_object!($name, $widget_type, ($($extends),*), (gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget $(,$implements)*), $(#[$metas])+ struct $struct_def, {
            fn class_init(klass: &mut Self::Class) {
                klass.bind_template();
                $($crate::call_func!(klass, $instance_callbacks);)?
            }
        
            fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
                obj.init_template();
            }
        });
    }
}

/*
Usage: quick_template!(ClassName, "path/to/template.ui", WidgetType (e.g., gtk::ScrolledWindow), (gtk::ExtendedWidgetsLike, gtk::Window, gtk::Widget), (gtk::ImplementedObjectsLike, gtk::Native, gtk::Root, gio::ShortcutMap), [props] [handlers] struct {
    structure definition here
    
    props or handlers may be inserted before the struct definition for:

    props - Allow custom properties (like #[property(get, set)])
    handlers - Allow custom template instance callbacks (like #[template_callback])
});

It's meant to quickly fill in all the boilerplate so you can just write the code.
*/
// TODO: Get rid of properties argument. The caller can set that up themselves with meta stuff (Just make the derive arg a list)
#[macro_export]
macro_rules! quick_template {
    ($name:ident, $resource_path:literal, $widget_type:ty, ($($extends:ty),*), ($($implements:ty),*)) => {
        $crate::full_template!($name, $widget_type, ($($extends),*), ($($implements),*), #[derive(CompositeTemplate, Default)] #[template(resource=$resource_path)] struct {}, ());
    };
    ($name:ident, $resource_path:literal, $widget_type:ty, ($($extends:ty),*), ($($implements:ty),*), $(#[$metas:meta])+ struct $struct_def : tt) => {
        $crate::full_template!($name, $widget_type, ($($extends),*), ($($implements),*), $(#[$metas])+ #[template(resource=$resource_path)] struct $struct_def, ());
    };
    ($name:ident, $resource_path:literal, $widget_type:ty, ($($extends:ty),*), ($($implements:ty),*), $(#[$metas:meta])+ handlers struct $struct_def : tt) => {
        $crate::full_template!($name, $widget_type, ($($extends),*), ($($implements),*), $(#[$metas])+ #[template(resource=$resource_path)] struct $struct_def, (bind_template_instance_callbacks));
    };
}
