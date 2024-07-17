use gpui::{actions, AppContext, KeyBinding, Menu, MenuItem};
use tracing::{debug, info};

actions!(muzak, [Quit]);

pub fn register_actions(cx: &mut AppContext) {
    debug!("registering actions");
    cx.on_action(|_: &Quit, cx| {
        println!("Quitting...");
        cx.quit()
    });
    debug!("actions: {:?}", cx.all_action_names());
    debug!("action available: {:?}", cx.is_action_available(&Quit));
    cx.bind_keys([KeyBinding::new("w", Quit, None)]);
    cx.set_menus(vec![Menu {
        name: "Image",
        items: vec![MenuItem::action("Quit", Quit)],
    }]);

    cx.observe_keystrokes(|ev, _| {
        debug!("keystroke observed, ev: {:?}", ev);
    })
    .detach();
}
