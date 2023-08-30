/* TODO:
* Maybe some way to grab what we need to automatically @implements and @extends?
* Add options for Properties, Signals, Callbacks?
*/

#[macro_export]
macro_rules! quick_template {
    ($name:ident, $resource_path:literal, $widget_type:ty, ($($extends:ty),*), ($($implements:ty),*), {$($implementation:item)+}) => {
        use gtk::{subclass::prelude::*, glib, CompositeTemplate};

        mod imp {
            use super::*;

            #[derive(Default, CompositeTemplate)]
            #[template(resource=$resource_path)]
            pub struct $name {}

            #[glib::object_subclass]
            impl ObjectSubclass for $name {
                const NAME : &'static str = concat!("JCC", stringify!($name));
                type Type = super::$name;
                type ParentType = $widget_type;

                fn class_init(klass: &mut Self::Class) {
                    klass.bind_template();
                }
            
                fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
                    obj.init_template();
                }
            }

            $($implementation)+
        }

        glib::wrapper! {
            pub struct $name(ObjectSubclass<imp::$name>) @extends $widget_type, gtk::Widget, $($extends),* @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, $($implements),*;
        }
    };
}