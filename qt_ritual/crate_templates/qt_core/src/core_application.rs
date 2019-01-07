include_generated!();

/// A struct providing valid `argc` and `argv` values for Qt application
/// objects.
///
/// Constructors of `qt_core::core_application::CoreApplication`,
/// `qt_gui::gui_application::GuiApplication` and `qt_widgets::application::Application`
/// require `argc` and `argv` values that are available in C++'s `main` function but
/// not available in Rust. More importantly, `argc` and `argv` must be valid for the entire
/// life of the application. This struct stores list of arguments in a format compatible with
/// `argc` and `argv`, and can be used to initialize Qt application objects.
/// `CoreApplicationArgs` must live longer than the application object.
///
/// `CoreApplication::create_and_exit` convenience function
/// and similar functions in the other application types
/// can be used instead of `CoreApplicationArgs`.
pub struct CoreApplicationArgs {
  _values: Vec<Vec<u8>>,
  argc: Box<::libc::c_int>,
  argv: Vec<*mut ::libc::c_char>,
}

impl CoreApplicationArgs {
  /// Creates an object containing `args`.
  pub fn from(mut args: Vec<Vec<u8>>) -> CoreApplicationArgs {
    for arg in &mut args {
      if !arg.ends_with(&[0]) {
        arg.push(0);
      }
    }
    CoreApplicationArgs {
      argc: Box::new(args.len() as ::libc::c_int),
      argv: args
        .iter_mut()
        .map(|x| x.as_mut_ptr() as *mut ::libc::c_char)
        .collect(),
      _values: args,
    }
  }
  /// Creates an object containing empty list of arguments.
  /// Although this is the cheapest way to construct a `CoreApplicationArgs`
  /// object, it's not clear whether Qt considers empty arguments list valid.
  pub fn empty() -> CoreApplicationArgs {
    CoreApplicationArgs::from(Vec::new())
  }

  /// Returns `(argc, argv)` values in the form accepted by the application objects'
  /// constructors.
  pub fn get(&mut self) -> (&mut ::libc::c_int, *mut *mut ::libc::c_char) {
    (self.argc.as_mut(), self.argv.as_mut_ptr())
  }

  #[cfg(unix)]
  /// Creates an object representing real arguments of the application.
  /// On Windows, this function uses empty argument list for performance reasons because
  /// Qt doesn't use `argc` and `argv` on Windows at all.
  pub fn from_real() -> CoreApplicationArgs {
    use std::os::unix::ffi::OsStringExt;
    let args = ::std::env::args_os().map(|arg| arg.into_vec()).collect();
    CoreApplicationArgs::from(args)
  }
  #[cfg(windows)]
  /// Creates an object representing real arguments of the application.
  /// On Windows, this function uses empty argument list for performance reasons because
  /// Qt doesn't use `argc` and `argv` on Windows at all.
  pub fn from_real() -> CoreApplicationArgs {
    // Qt doesn't use argc and argv on Windows anyway
    // TODO: check this
    CoreApplicationArgs::empty()
  }
}

impl ::core_application::CoreApplication {
  /// A convenience function for performing proper initialization and de-initialization of
  /// a Qt application.
  ///
  /// This function creates `CoreApplication` with valid `argc` and `argv`, calls the passed
  /// closure `f(app)` with the application object and exist the process with the exit code
  /// returned by the closure. The closure should perform the initialization of the application
  /// and either return immediately or call `CoreApplication::exec()` and return its return value:
  /// ```
  /// fn main() {
  ///   CoreApplication::create_and_exit(|app| {
  ///     // initialization goes here
  ///     CoreApplication::exec()
  ///   })
  /// }
  /// ```
  pub fn create_and_exit<F: FnOnce(&mut ::core_application::CoreApplication) -> i32>(f: F) -> ! {
    let exit_code = {
      let mut args = CoreApplicationArgs::from_real();
      let mut app = unsafe { ::core_application::CoreApplication::new(args.get()) };
      f(app.as_mut())
    };
    ::std::process::exit(exit_code)
  }
}
