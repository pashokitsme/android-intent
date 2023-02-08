use jni::{
    objects::{JObject, JString},
    AttachGuard, JavaVM,
};

pub fn current_vm() -> JavaVM {
    let cx = ndk_context::android_context();
    unsafe { JavaVM::from_raw(cx.vm().cast()) }.unwrap()
}

pub enum Action {
    View,
    Edit,
}

impl AsRef<str> for Action {
    fn as_ref(&self) -> &str {
        match self {
            Self::View => "ACTION_VIEW",
            Self::Edit => "ACTION_EDIT",
        }
    }
}

#[must_use]
pub struct Intent<'env> {
    env: AttachGuard<'env>,
    object: JObject<'env>,
}

impl<'env> Intent<'env> {
    pub fn new(vm: &'env JavaVM, action: impl AsRef<str>) -> Self {
        let env = vm.attach_current_thread().unwrap();

        let intent_class = env.find_class("android/content/Intent").unwrap();
        let action_view = env
            .get_static_field(intent_class, action.as_ref(), "Ljava/lang/String;")
            .unwrap();

        let intent = env
            .new_object(intent_class, "(Ljava/lang/String;)V", &[action_view.into()])
            .unwrap();

        Self {
            env,
            object: intent,
        }
    }

    pub fn new_with_uri(vm: &'env JavaVM, action: impl AsRef<str>, uri: impl AsRef<str>) -> Self {
        let env = vm.attach_current_thread().unwrap();

        let url_string = env.new_string(uri).unwrap();
        let uri_class = env.find_class("android/net/Uri").unwrap();
        let uri = env
            .call_static_method(
                uri_class,
                "parse",
                "(Ljava/lang/String;)Landroid/net/Uri;",
                &[JString::from(url_string).into()],
            )
            .unwrap();

        let intent_class = env.find_class("android/content/Intent").unwrap();
        let action_view = env
            .get_static_field(intent_class, action.as_ref(), "Ljava/lang/String;")
            .unwrap();

        let intent = env
            .new_object(
                intent_class,
                "(Ljava/lang/String;Landroid/net/Uri;)V",
                &[action_view.into(), uri.into()],
            )
            .unwrap();

        Self {
            env,
            object: intent,
        }
    }

    pub fn push_extra(&self, key: impl AsRef<str>, value: impl AsRef<str>) {
        let key = self.env.new_string(key).unwrap();
        let value = self.env.new_string(value).unwrap();

        self.env
            .call_method(
                self.object,
                "putExtra",
                "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/Intent;",
                &[key.into(), value.into()],
            )
            .unwrap();
    }

    pub fn with_extra(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> &Self {
        self.push_extra(key, value);
        self
    }

    pub fn set_chooser(&mut self, title: impl AsRef<str>) -> &mut Self {
        let jstring = self.env.new_string(title).unwrap();

        let intent_class = self.env.find_class("android/content/Intent").unwrap();
        let intent = self
            .env
            .call_static_method(
                intent_class,
                "createChooser",
                "(Ljava/lang/String;Landroid/net/Uri;)V",
                &[self.object.into(), jstring.into()],
            )
            .unwrap();

        self.object = intent.try_into().unwrap();
        self
    }

    pub fn into_chooser(&mut self, title: impl AsRef<str>) -> &mut Self {
        let jstring = self.env.new_string(title).unwrap();

        let intent_class = self.env.find_class("android/content/Intent").unwrap();
        let intent = self
            .env
            .call_static_method(
                intent_class,
                "createChooser",
                "(Ljava/lang/String;Landroid/net/Uri;)V",
                &[self.object.into(), jstring.into()],
            )
            .unwrap();

        self.object = intent.try_into().unwrap();
        self
    }

    pub fn set_type(&self, type_name: impl AsRef<str>) {
        let jstring = self.env.new_string(type_name).unwrap();

        self.env
            .call_method(
                self.object,
                "setType",
                "(Ljava/lang/String;)Landroid/content/Intent;",
                &[jstring.into()],
            )
            .unwrap();
    }

    pub fn with_type(&self, type_name: impl AsRef<str>) -> &Self {
        self.set_type(type_name);
        self
    }

    pub fn start_activity(&self) {
        let cx = ndk_context::android_context();
        let activity = unsafe { JObject::from_raw(cx.context() as jni::sys::jobject) };

        self.env
            .call_method(
                activity,
                "startActivity",
                "(Landroid/content/Intent;)V",
                &[self.object.into()],
            )
            .unwrap();
    }
}
