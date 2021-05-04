use proc_macro::TokenStream;

#[proc_macro]
pub fn start_threads(_tokens: TokenStream) -> TokenStream {
    let threads: i32 = std::env::var("THREADS").expect("Can't find THREADS env var").parse().expect("THREADS has to be a number");
    let particles: i32 = std::env::var("PARTICLES").expect("Can't find PARTICLES env var").parse().expect("PARTICLES has to be a number");

    let mut src = String::new();
    
    assert!(particles % threads == 0);

    src.push_str("{
        let mut handles = vec![];");


    for i in 0..threads {
        src.push_str(
            &format!("handles.push(std::thread::spawn(|| {{
                                const THREAD_NUM: usize = {};
                                {}
                            }}
                        ));", i, _tokens)
        );
    }

    src.push_str("handles }");

    src.parse().expect("Start Threads Proc Macro Failed")
}