/* TODO:
* Maybe some way to grab what we need to automatically @implements and @extends?
* Add options for Properties, Signals, Callbacks?
*/

/*
use things_to_use;

mod imp {
    use super::*;

    IF PROPERTIES:
    #[derive(Properties)]
    #[properties(wrapper_type=super:$name)]
    REGULAR:
    #[derive(Default, CompositeTemplate)]
    #[template(resource=$resource_path)]
    pub struct $name $STRUCT_DEF

    #[glib::object_subclass]
    impl ObjectSubclass for ContentList {
        const NAME : &'static str = "JCCContentList";
        type Type = super::ContentList;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            Is this optional? Required for signal callbacks though.
			klass.bind_template_instance_callbacks();
        }
    
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    IF PROPERTIES:
    #[derived_properties]
    impl ObjectImpl for ContentList $object_impl;
    for each $extends:
    impl $($extends)Impl for ContentList {}
    impl $widget_typeImpl for $name {}
}

glib::wrapper! {
    pub struct ContentList(ObjectSubclass<imp::ContentList>) @extends $widget_type, $EXTENDS, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Scrollable;
}

*/

#[macro_export(local_inner_macros)]
macro_rules! call_func {
    ($obj:ident, $func:ident) => {
        $obj.$func()
    };
}

#[macro_export(local_inner_macros)]
macro_rules! derive {
    ($($derives:ident),+) => {
        #[derive($($derives),+)]
    }
}

#[macro_export]
macro_rules! full_template {
    ($name:ident, ($($resource_path:literal)?), $struct_def:tt, $widget_type:ty, ($($extends:ty),*), ($($implements:ty),*), #[$derive:meta], ($($properties:meta)?), ($($instance_callbacks:ident)?)) => {
        use gtk::{subclass::prelude::*, glib, CompositeTemplate, prelude::*};
        use glib::Object;

        mod imp {
            use super::*;

            // $crate::derive!{$($derives),+}
            #[$derive]
            $(#[template(resource=$resource_path)])?
            $(#[$properties])?
            pub struct $name $struct_def

            #[glib::object_subclass]
            impl ObjectSubclass for $name {
                const NAME : &'static str = concat!("JCC", stringify!($name));
                type Type = super::$name;
                type ParentType = $widget_type;

                fn class_init(klass: &mut Self::Class) {
                    klass.bind_template();
                    $($crate::call_func!(klass, $instance_callbacks);)?
                }
            
                fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
                    obj.init_template();
                }
            }
        }

        glib::wrapper! {
            pub struct $name(ObjectSubclass<imp::$name>) @extends $widget_type, $($extends,)* @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget $(,$implements)*;
        }
    };
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
#[macro_export]
macro_rules! quick_template {
    ($name:ident, $widget_type:ty, ($($extends:ty),*), ($($implements:ty),*), struct $struct_def : tt) => {
        $crate::full_template!($name, (), $struct_def, $widget_type, ($($extends),*), ($($implements),*), (), (), ());
    };
    ($name:ident, $resource_path:literal, $widget_type:ty, ($($extends:ty),*), ($($implements:ty),*)) => {
        $crate::full_template!($name, ($resource_path), {}, $widget_type, ($($extends),*), ($($implements),*), #[derive(CompositeTemplate, Default)], (), ());
    };
    ($name:ident, $resource_path:literal, $widget_type:ty, ($($extends:ty),*), ($($implements:ty),*), #[$derive:meta] struct $struct_def : tt) => {
        $crate::full_template!($name, ($resource_path), $struct_def, $widget_type, ($($extends),*), ($($implements),*), #[$derive], (), ());
    };
    ($name:ident, $resource_path:literal, $widget_type:ty, ($($extends:ty),*), ($($implements:ty),*), #[$derive:meta] props struct $struct_def : tt) => {
        $crate::full_template!($name, ($resource_path), $struct_def, $widget_type, ($($extends),*), ($($implements),*), #[$derive], (properties(wrapper_type=super::$name)), ());
    };
    ($name:ident, $resource_path:literal, $widget_type:ty, ($($extends:ty),*), ($($implements:ty),*), #[$derive:meta] props handlers struct $struct_def : tt) => {
        $crate::full_template!($name, ($resource_path), $struct_def, $widget_type, ($($extends),*), ($($implements),*), #[$derive], (properties(wrapper_type=super::$name)), (bind_template_instance_callbacks));
    };
    ($name:ident, $resource_path:literal, $widget_type:ty, ($($extends:ty),*), ($($implements:ty),*), #[$derive:meta] handlers struct $struct_def : tt) => {
        $crate::full_template!($name, ($resource_path), $struct_def, $widget_type, ($($extends),*), ($($implements),*), #[$derive], (), (bind_template_instance_callbacks));
    };
}
