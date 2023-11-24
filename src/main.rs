use winsafe::{co, COLORREF, gui};
use winsafe::co::{CDDS, LVS, LVS_EX};
use winsafe::gui::{Horz, ListViewOpts, Vert};
use winsafe::prelude::{GuiEvents, GuiNativeControlEvents, GuiParent, IntUnderlying};

#[derive(Clone)]
pub(crate) struct MainWindow {
    pub(crate) wnd: gui::WindowMain,
    list_view: gui::ListView,
}

impl MainWindow {
    pub fn new() -> Self {
        let wnd = gui::WindowMain::new(
            // instantiate the window manager
            gui::WindowMainOpts {
                title: "Main Window".to_owned(),
                size: (900, 600),
                style: gui::WindowMainOpts::default().style
                    | co::WS::MINIMIZEBOX
                    | co::WS::MAXIMIZEBOX
                    | co::WS::SIZEBOX,
                ..Default::default() // leave all other options as default
            },
        );

        let list_view = gui::ListView::new(
            &wnd,
            ListViewOpts {
                position: (10, 10),
                size: (880, 580),
                columns: vec![("Text".to_string(), 3200)],
                resize_behavior: (Horz::Resize, Vert::Resize),
                list_view_ex_style: LVS_EX::DOUBLEBUFFER | LVS_EX::FULLROWSELECT,
                list_view_style: LVS::REPORT
                    | LVS::NOLABELWRAP
                    | LVS::SHOWSELALWAYS,
                ..Default::default()
            },
        );

        let mut myself = Self {
            wnd,
            list_view,
        };
        myself.events();
        myself
    }

    fn events(&mut self) {

        self.wnd.on().wm_create({
            let myself = self.clone();
            move |_msg| {

                // add some test items
                myself.list_view.items().add(
                    &["I have changed colors in debug and release using unsafe"], None
                );

                myself.list_view.items().add(
                    &["I work only in debug mode. 'cargo run --release' optimizes my new colors away"], None
                );

                myself.list_view.items().add(
                    &["opt-level = 1 is enough. try 'cargo run --profile smallopt'"], None
                );

                Ok(0)
            }
        });



        self.list_view.on().nm_custom_draw({
            move |draw: &mut winsafe::NMLVCUSTOMDRAW| {
                match draw.mcd.dwDrawStage {
                    CDDS::PREPAINT => {
                        Ok(co::CDRF::NOTIFYITEMDRAW)
                    }
                    CDDS::ITEMPREPAINT => {
                        let txt_clr = COLORREF::new(0xff, 0xff, 0xff);
                        let bg_clr = COLORREF::new(0x00, 0x00, 0xff);

                        if draw.mcd.dwItemSpec == 0 {

                            unsafe {
                                *draw.clrText.as_mut() = txt_clr.raw();
                            }
                            unsafe {
                                *draw.clrTextBk.as_mut() = bg_clr.raw();
                            }

                        } else {
                            draw.clrText = txt_clr;
                            draw.clrTextBk = bg_clr;
                        }

                        Ok(co::CDRF::DODEFAULT)
                    }
                    _ => Ok(co::CDRF::DODEFAULT),
                }
            }
        });
    }
}

fn main() {
    let my = MainWindow::new(); // instantiate our main window
    if let Err(e) = my.wnd.run_main(None) {
        // ... and run it
        eprintln!("{}", e);
    }
}
