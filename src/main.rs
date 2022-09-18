mod solver;
use dioxus::{ prelude::*, desktop::tao::dpi::LogicalSize };

fn main() {
    dioxus::desktop::launch_cfg(app, |cfg| {
        cfg.with_window(|w| {
            w.with_resizable(false)
                .with_title("Nerdle Solver")
                .with_inner_size(LogicalSize::new(600, 600))
        })
        .with_disable_context_menu(true)
    });
}

fn app(cx: Scope) -> Element {
    let mut eq = use_state(&cx, || String::new());
    let mut clues = use_state(&cx, || String::new());
    let mut result = use_state(&cx, || String::new());
    
    cx.render(rsx!(
        style { [include_str!("./css/style.css")] }
        h1 {  // Title
            style: "margin-top:5vh",
            "🤓 Nerdle Solver 🤓"
        }
        h5 {  // Textbox 1 label
			style: "margin-top: 15vh",
            "Enter your initial guess:"
        }
        input {  // Textbox 1
            oninput: |txt| {
                eq.set(txt.value.clone());
            },
			style: "margin-top:-10px"
		}
		h5 {  // Textbox 2 label
			style: "margin-top: 10vh",
            "Enter all your clues:"
        }
        input {  // Textbox 2
            oninput: |txt| {
                clues.set(txt.value.clone());
            },
			style: "margin-top:-10px"
		}
		button {   // Calculate button
			style: "margin-top: 10vh",
			p {
				style: "text-align:center",
				onclick: |_| {
				    result.set(solver::final_calculate(eq.get(), clues.get()));
				},
				"Calculate"
			}
		}
		p {  // Hidden error text
			class: "errortext",
			"Invalid input!"
		}
		// Stats section
		p {
			class: "stattext",
			style: "margin-top:7vh",
			"Output:"
		}
		p {
			class: "stattext",
			style: "margin-top:-10px",
			[result.get().as_str()]
		}
		// p {
		// 	class: "stattext",
		// 	"max depth/iterations:"
		// }
		// p {
		// 	class: "stattext",
		// 	style: "margin-top:-10px",
		// 	[format!("{}/{}", 5, 6)]
		// }
    ))
}