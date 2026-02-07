use yew::prelude::*;
use yew_router::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;

// --- 1. ROUTING CONFIG ---
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")] Home,
    #[at("/mba")] Mba,
    #[at("/upsc")] Upsc,
    #[at("/jee")] Jee,
    #[at("/ca")] Ca,
    #[at("/tech")] Tech,
}

// --- 2. DATA MODELS ---
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Challenge {
    pub id: i32,
    pub date: String,
    pub title: String,
    pub icon: String,
    pub category: String,
    pub hint: String,
    pub target: String,
}

// --- 3. REUSABLE QUEST ENGINE ---
#[derive(Properties, PartialEq)]
pub struct QuestProps {
    pub persona: String,
    pub challenges: Vec<Challenge>,
}

#[function_component(QuestEngine)]
fn quest_engine(props: &QuestProps) -> Html {
    let storage_key = format!("val_prog_{}", props.persona.to_lowercase());
    let view = use_state(|| "menu".to_string());
    let solved = use_state(|| LocalStorage::get::<Vec<i32>>(&storage_key).unwrap_or_default());
    let active_id = use_state(|| 1);
    let user_input = use_state(|| "".to_string());
    let error_msg = use_state(|| "".to_string());

    let on_execute = {
        let view = view.clone();
        let solved = solved.clone();
        let user_input = user_input.clone();
        let error_msg = error_msg.clone();
        let active_id = active_id.clone();
        let challenges = props.challenges.clone();
        let storage_key = storage_key.clone();

        Callback::from(move |_: ()| {
            let day = challenges.iter().find(|c| c.id == *active_id).unwrap();
            let input = user_input.trim().to_lowercase();
            if input == day.target.to_lowercase() {
                let mut current = (*solved).clone();
                if !current.contains(&day.id) {
                    current.push(day.id);
                    let _ = LocalStorage::set(&storage_key, &current);
                    solved.set(current);
                }
                user_input.set("".into());
                view.set("menu".into());
            } else {
                error_msg.set("⚠️ Verification failed. Incorrect sequence.".into());
            }
        })
    };

    html! {
        <div class="min-h-screen bg-[#1a0a0d] text-[#ffccd5] font-mono flex items-center justify-center p-4">
            <div class="w-full max-w-lg bg-[#2d0f14] rounded-2xl overflow-hidden border border-[#ff4d6d]/20 shadow-2xl flex flex-col h-[90vh] md:h-auto">
                <div class="bg-[#1a0a0d] p-4 border-b border-[#590d22] flex items-center justify-between">
                    <span class="text-[10px] text-rose-300/40 uppercase tracking-widest">{format!("{}_quest.sh", props.persona)}</span>
                    <div class="flex gap-1.5"><div class="w-2 h-2 rounded-full bg-[#ff4d6d]/40"/><div class="w-2 h-2 rounded-full bg-[#c9184a]/40"/></div>
                </div>

                <div class="p-6 overflow-y-auto flex-grow">
                    /* 
                       FIX: We wrap the whole if/else in braces { } 
                       This allows us to write standard Rust logic inside.
                    */
                    {
                        if *view == "menu" {
                            html! {
                                <div class="text-center">
                                    <h1 class="text-2xl font-bold text-white mb-1 uppercase tracking-tighter italic">{&props.persona}{" Quest"}</h1>
                                    <p class="text-rose-300/60 text-[10px] mb-8">{"// Solve 8 days to unlock Valentine Cards"}</p>
                                    <div class="grid grid-cols-2 gap-4">
                                        { for props.challenges.iter().map(|c| {
                                            let id = c.id; let v = view.clone(); let aid = active_id.clone(); let is_solved = solved.contains(&id);
                                            html! {
                                                <div onclick={move |_| { aid.set(id); v.set("play".into()); }}
                                                     class={format!("p-5 rounded-xl border transition-all cursor-pointer flex flex-col items-center {}", 
                                                     if is_solved { "border-[#ff4d6d]/50 bg-[#590d22]/30 shadow-inner" } else { "border-white/5 hover:border-[#ff4d6d]/40" })}>
                                                    <span class="text-3xl mb-1">{ &c.icon }</span>
                                                    <span class="text-[10px] text-rose-300/40 uppercase">{ &c.date }</span>
                                                    if is_solved { <span class="text-[8px] text-[#ff4d6d] font-bold mt-1">{"[UNLOCKED]"}</span> }
                                                </div>
                                            }
                                        })}
                                    </div>
                                    <div class="mt-8"><Link<Route> to={Route::Home} classes="text-[10px] text-slate-600 underline">{"[ Exit to Main Menu ]"}</Link<Route>></div>
                                </div>
                            }
                        } else {
                            // Logic is now allowed here!
                            let c = props.challenges.iter().find(|x| x.id == *active_id).unwrap().clone();
                            html! {
                                <div class="flex flex-col h-full text-center">
                                    <span class="px-3 py-1 bg-[#590d22]/40 rounded-full text-[10px] text-[#ffccd5] border border-[#ff4d6d]/20 mb-6">{&c.date}{" - "}{&c.title}</span>
                                    <div class="bg-[#1a0a11] p-6 rounded-lg border border-[#590d22] mb-6 text-left">
                                        <div class="text-[10px] text-rose-300/30 mb-2 uppercase tracking-widest">{"Field: "}{&c.category}</div>
                                        <p class="text-white text-sm mb-6">{&c.hint}</p>
                                        <div class="relative flex items-center border-b border-[#590d22] focus-within:border-[#ff4d6d] transition">
                                            <span class="text-[#ff4d6d] font-bold mr-2">{">"}</span>
                                            <input class="w-full bg-transparent outline-none text-white text-sm py-2" placeholder="Analyze..." autofocus=true
                                                value={(*user_input).clone()}
                                                oninput={let ui = user_input.clone(); let em = error_msg.clone(); move |e: InputEvent| { ui.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()); em.set("".into()); }}
                                                onkeydown={let oe = on_execute.clone(); move |e: KeyboardEvent| if e.key() == "Enter" { oe.emit(()); }}
                                            />
                                        </div>
                                    </div>
                                    if !(*error_msg).is_empty() { <p class="text-red-500 text-[10px] mb-4">{&*error_msg}</p> }
                                    <button onclick={move |_| on_execute.emit(())} class="w-full py-4 bg-gradient-to-r from-[#c9184a] to-[#ff4d6d] text-white rounded-xl font-bold shadow-lg shadow-black/20 active:scale-95 transition">{"VERIFY()"}</button>
                                    <button onclick={move |_| view.set("menu".into())} class="mt-4 text-rose-300/40 text-[10px]">{"← BACK"}</button>
                                </div>
                            }
                        }
                    }
                </div>
            </div>
        </div>
    }
}

// --- 4. BASE ROUTE (PORTAL) ---
#[function_component(Home)]
fn home() -> Html {
    // 1. Define the Cheesy Persona List
    // (Route, Icon, Cheesy Title, Indicative Subtitle, Storage Key)
    let personas = vec![
        (
            Route::Mba, "📈", 
            "The Strategic Partner", 
            "Maximizing our Net Present Valentine (NPV)", 
            "val_prog_mba"
        ),
        (
            Route::Upsc, "🏛️", 
            "The Constitutional Bond", 
            "You are the Preamble to my life", 
            "val_prog_upsc"
        ),
        (
            Route::Jee, "⚛️", 
            "The Molecular Attraction", 
            "Stronger than a Triple Covalent Bond", 
            "val_prog_jee"
        ),
        (
            Route::Ca, "📊", 
            "The Balanced Ledger", 
            "Auditing my heart for your entry", 
            "val_prog_ca"
        ),
        (
            Route::Tech, "💻", 
            "The Perfect Merge", 
            "Zero conflicts in our source code", 
            "val_prog_tech"
        ),
    ];

    html! {
        <div class="min-h-screen w-full bg-[#1a0a0d] flex flex-col items-center py-12 px-6 text-center">
            <div class="max-w-md w-full">
                /* Title Section */
                <h1 class="text-4xl font-bold text-[#ff4d6d] mb-2 italic tracking-tighter uppercase">{"Aspirant_Aura"}</h1>
                <p class="text-[#ffccd5] text-[10px] tracking-[0.3em] uppercase mb-12 opacity-60">{"// select your destiny"}</p>
                
                /* Scrollable Grid Area */
                <div class="grid gap-6 pb-16 w-full">
                    { for personas.into_iter().map(|(route, icon, label, desc, storage_key)| {
                        let solved_count = LocalStorage::get::<Vec<i32>>(storage_key).unwrap_or_default().len();
                        let progress_pct = (solved_count as f32 / 8.0) * 100.0;
                        let is_active = solved_count > 0;

                        html! {
                            <Link<Route> to={route} classes="block p-6 bg-[#2d0f14] glow-card rounded-2xl text-left group border border-[#ff4d6d]/10 transition-all duration-500 hover:-translate-y-1">
                                <div class="flex items-start gap-4">
                                    /* Icon with a romantic hover scale */
                                    <span class="text-4xl group-hover:rotate-12 transition duration-500">{icon}</span>
                                    
                                    <div class="flex-grow">
                                        <div class="flex justify-between items-start mb-1">
                                            <div>
                                                <div class="text-[#ffccd5] font-bold text-lg group-hover:text-[#ff4d6d] transition leading-tight">
                                                    {label}
                                                </div>
                                                <div class="text-[10px] text-rose-300/40 italic mt-1 leading-tight">
                                                    {desc}
                                                </div>
                                            </div>
                                            /* Indicative Pulse */
                                            if is_active {
                                                <div class="w-2 h-2 rounded-full bg-[#ff4d6d] animate-pulse shadow-[0_0_8px_#ff4d6d]"></div>
                                            }
                                        </div>
                                        
                                        /* Indicative Progress Visual */
                                        <div class="mt-5 space-y-1.5">
                                            <div class="flex justify-between items-end">
                                                <span class={format!("text-[8px] font-bold uppercase tracking-widest {}", if is_active { "text-[#ff4d6d]" } else { "text-white/10" })}>
                                                    { if solved_count == 8 { "Fully Decoded" } else if is_active { "Syncing Heart..." } else { "Link Offline" } }
                                                </span>
                                                <span class="text-[9px] text-rose-300/20 font-mono">
                                                    {format!("{:0>2}/08", solved_count)}
                                                </span>
                                            </div>
                                            /* Thin, elegant progress line */
                                            <div class="w-full bg-black/30 h-[1px] rounded-full overflow-hidden">
                                                <div class="bg-gradient-to-r from-[#590d22] to-[#ff4d6d] h-full transition-all duration-1000 ease-in-out" 
                                                     style={format!("width: {}%", progress_pct)}></div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </Link<Route>>
                        }
                    })}
                </div>
                
                /* Footer Visual */
                <div class="mt-auto pt-8 flex flex-col items-center gap-2 opacity-30">
                    <p class="text-[#ffccd5] text-[9px] uppercase tracking-[0.5em]">{"Connection Secure"}</p>
                    <div class="flex gap-1">
                        <div class="w-1 h-1 rounded-full bg-[#ff4d6d]"></div>
                        <div class="w-1 h-1 rounded-full bg-[#ff4d6d]/60"></div>
                        <div class="w-1 h-1 rounded-full bg-[#ff4d6d]/30"></div>
                    </div>
                </div>
            </div>
        </div>
    }
}
// --- 5. DATA GENERATORS ---
fn get_mba() -> Vec<Challenge> {
    vec![
        Challenge { id: 1, date: "Feb 7".into(), title: "Rose Marketing".into(), icon: "🌹".into(), category: "4 Ps".into(), hint: "The 'P' that deals with spreading the word and advertising your love?".into(), target: "Promotion".into() },
        Challenge { id: 2, date: "Feb 8".into(), title: "Value Proposal".into(), icon: "💍".into(), category: "Finance".into(), hint: "The capital needed for day-to-day operations: Current Assets minus Current Liabilities?".into(), target: "Working Capital".into() },
        Challenge { id: 3, date: "Feb 9".into(), title: "Supply Chain Sweetness".into(), icon: "🍫".into(), category: "Logistics".into(), hint: "The 'Last ____' delivery: the most expensive and final leg of the supply chain.".into(), target: "Mile".into() },
        Challenge { id: 4, date: "Feb 10".into(), title: "Strategic Teddy".into(), icon: "🧸".into(), category: "Analysis".into(), hint: "A framework for Strengths, Weaknesses, Opportunities, and Threats.".into(), target: "SWOT".into() },
        Challenge { id: 5, date: "Feb 11".into(), title: "Promise Equity".into(), icon: "🤝".into(), category: "Stock Market".into(), hint: "An invitation to the public to buy shares for the first time (Acronym).".into(), target: "IPO".into() },
        Challenge { id: 6, date: "Feb 12".into(), title: "Economic Embrace".into(), icon: "🤗".into(), category: "Micro-Econ".into(), hint: "When Supply equals Demand, the market is in this state.".into(), target: "Equilibrium".into() },
        Challenge { id: 7, date: "Feb 13".into(), title: "Kiss the P&L".into(), icon: "💋".into(), category: "Accounting".into(), hint: "The term for 'Earnings Before Interest and Taxes' (Acronym).".into(), target: "EBIT".into() },
        Challenge { id: 8, date: "Feb 14".into(), title: "Placement Day".into(), icon: "💖".into(), category: "Leadership".into(), hint: "The 'C' level role that manages human capital (Acronym).".into(), target: "CHRO".into() },
    ]
}

fn get_upsc() -> Vec<Challenge> {
    vec![
        Challenge { id: 1, date: "Feb 7".into(), title: "Historical Rose".into(), icon: "🌹".into(), category: "Modern History".into(), hint: "The movement launched by Gandhiji in 1942: 'Quit ____'.".into(), target: "India".into() },
        Challenge { id: 2, date: "Feb 8".into(), title: "Constitutional Bond".into(), icon: "💍".into(), category: "Polity".into(), hint: "Which Article of the Constitution guarantees the 'Right to Life'?".into(), target: "Article 21".into() },
        Challenge { id: 3, date: "Feb 9".into(), title: "Sweet Plateau".into(), icon: "🍫".into(), category: "Geography".into(), hint: "The large triangular plateau that covers most of South India.".into(), target: "Deccan".into() },
        Challenge { id: 4, date: "Feb 10".into(), title: "Bureaucratic Teddy".into(), icon: "🧸".into(), category: "Ethics".into(), hint: "The value of being honest and having strong moral principles.".into(), target: "Integrity".into() },
        Challenge { id: 5, date: "Feb 11".into(), title: "Promissory Directive".into(), icon: "🤝".into(), category: "Polity".into(), hint: "Acronym for the Directive Principles of State Policy.".into(), target: "DPSP".into() },
        Challenge { id: 6, date: "Feb 12".into(), title: "Global Hug".into(), icon: "🤗".into(), category: "International Rel".into(), hint: "The permanent headquarters of SAARC is located in this city.".into(), target: "Kathmandu".into() },
        Challenge { id: 7, date: "Feb 13".into(), title: "Monetary Kiss".into(), icon: "💋".into(), category: "Economy".into(), hint: "The tax levied on the 'Value Added' at each stage of production.".into(), target: "VAT".into() },
        Challenge { id: 8, date: "Feb 14".into(), title: "LBSNAA Final".into(), icon: "💖".into(), category: "Preamble".into(), hint: "The first word of the Preamble of the Indian Constitution.".into(), target: "We".into() },
    ]
}

fn get_jee() -> Vec<Challenge> {
    vec![
        Challenge { id: 1, date: "Feb 7".into(), title: "Atomic Rose".into(), icon: "🌹".into(), category: "Organic Chem".into(), hint: "The functional group -OH represents which class of compounds?".into(), target: "Alcohol".into() },
        Challenge { id: 2, date: "Feb 8".into(), title: "Kinematic Proposal".into(), icon: "💍".into(), category: "Physics".into(), hint: "The rate of change of momentum is called?".into(), target: "Force".into() },
        Challenge { id: 3, date: "Feb 9".into(), title: "Calculus Cocoa".into(), icon: "🍫".into(), category: "Mathematics".into(), hint: "The integral of 1/x dx is the natural ____ of x.".into(), target: "log".into() },
        Challenge { id: 4, date: "Feb 10".into(), title: "Quantum Teddy".into(), icon: "🧸".into(), category: "Physics".into(), hint: "The principle that says you can't know Position and Velocity simultaneously.".into(), target: "Heisenberg".into() },
        Challenge { id: 5, date: "Feb 11".into(), title: "Covalent Promise".into(), icon: "🤝".into(), category: "Inorganic Chem".into(), hint: "Bonds formed by the actual transfer of electrons.".into(), target: "Ionic".into() },
        Challenge { id: 6, date: "Feb 12".into(), title: "Thermodynamic Hug".into(), icon: "🤗".into(), category: "Physics".into(), hint: "The measure of disorder or randomness in a system.".into(), target: "Entropy".into() },
        Challenge { id: 7, date: "Feb 13".into(), title: "Projectile Kiss".into(), icon: "💋".into(), category: "Mechanics".into(), hint: "At what angle (in degrees) is the range of a projectile maximum?".into(), target: "45".into() },
        Challenge { id: 8, date: "Feb 14".into(), title: "IIT Bombay Gate".into(), icon: "💖".into(), category: "Mathematics".into(), hint: "A matrix whose determinant is zero is called a ____ matrix.".into(), target: "Singular".into() },
    ]
}

fn get_ca() -> Vec<Challenge> {
    vec![
        Challenge { id: 1, date: "Feb 7".into(), title: "Debit the Rose".into(), icon: "🌹".into(), category: "Accounts".into(), hint: "Personal Account Rule: Debit the Receiver, Credit the ____?".into(), target: "Giver".into() },
        Challenge { id: 2, date: "Feb 8".into(), title: "Audit Proposal".into(), icon: "💍".into(), category: "Audit".into(), hint: "The systematic examination of books of accounts is called?".into(), target: "Auditing".into() },
        Challenge { id: 3, date: "Feb 9".into(), title: "Taxing Chocolate".into(), icon: "🍫".into(), category: "Direct Tax".into(), hint: "The unique 10-digit alphanumeric number issued by the IT Dept.".into(), target: "PAN".into() },
        Challenge { id: 4, date: "Feb 10".into(), title: "Legal Teddy".into(), icon: "🧸".into(), category: "Law".into(), hint: "An agreement enforceable by law is called a ____?".into(), target: "Contract".into() },
        Challenge { id: 5, date: "Feb 11".into(), title: "Accrual Promise".into(), icon: "🤝".into(), category: "Standards".into(), hint: "The accounting concept that income is recorded when earned, not when received.".into(), target: "Accrual".into() },
        Challenge { id: 6, date: "Feb 12".into(), title: "Depreciation Hug".into(), icon: "🤗".into(), category: "Accounting".into(), hint: "The SLM full form: ____ Line Method.".into(), target: "Straight".into() },
        Challenge { id: 7, date: "Feb 13".into(), title: "Liquidity Kiss".into(), icon: "💋".into(), category: "FM".into(), hint: "Quick Assets / Current Liabilities is the ____ Ratio.".into(), target: "Liquid".into() },
        Challenge { id: 8, date: "Feb 14".into(), title: "Balance Sheet Match".into(), icon: "💖".into(), category: "Final".into(), hint: "The standard audit opinion that everything is fair: ____ Opinion.".into(), target: "Unqualified".into() },
    ]
}

fn get_tech() -> Vec<Challenge> {
    vec![
        Challenge { id: 1, date: "Feb 7".into(), title: "git send-rose".into(), icon: "🌹".into(), category: "Git".into(), hint: "Command to create a new branch and switch to it immediately.".into(), target: "git checkout -b".into() },
        Challenge { id: 2, date: "Feb 8".into(), title: "Query Proposal".into(), icon: "💍".into(), category: "SQL".into(), hint: "The keyword used to sort the result-set in ascending or descending order.".into(), target: "ORDER BY".into() },
        Challenge { id: 3, date: "Feb 9".into(), title: "🍫 Array".into(), icon: "🍫".into(), category: "JavaScript".into(), hint: "The array method used to create a new array by transforming every element.".into(), target: "map".into() },
        Challenge { id: 4, date: "Feb 10".into(), title: "<Teddy />".into(), icon: "🧸".into(), category: "React".into(), hint: "The prop used to uniquely identify elements in a list.".into(), target: "key".into() },
        Challenge { id: 5, date: "Feb 11".into(), title: "await Promise()".into(), icon: "🤝".into(), category: "Backend".into(), hint: "Status code for 'Not Found'.".into(), target: "404".into() },
        Challenge { id: 6, date: "Feb 12".into(), title: "let mut hug".into(), icon: "🤗".into(), category: "Rust".into(), hint: "The smart pointer used for heap allocation (3 letters).".into(), target: "Box".into() },
        Challenge { id: 7, date: "Feb 13".into(), title: "Kiss.rb".into(), icon: "💋".into(), category: "Ruby".into(), hint: "The popular web framework written in Ruby.".into(), target: "Rails".into() },
        Challenge { id: 8, date: "Feb 14".into(), title: "Merge Conflict".into(), icon: "💖".into(), category: "DevOps".into(), hint: "The automated process of building, testing, and deploying: CI/____.".into(), target: "CD".into() },
    ]
}

// --- 6. ROOT APP ---
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={move |route| match route {
                Route::Home => html! { <Home /> },
                Route::Mba => html! { <QuestEngine persona="MBA" challenges={get_mba()} /> },
                Route::Upsc => html! { <QuestEngine persona="UPSC" challenges={get_upsc()} /> },
                Route::Jee => html! { <QuestEngine persona="JEE" challenges={get_jee()} /> },
                Route::Ca => html! { <QuestEngine persona="CA" challenges={get_ca()} /> },
                Route::Tech => html! { <QuestEngine persona="Techie" challenges={get_tech()} /> },
            }} />
        </BrowserRouter>
    }
}

fn main() { yew::Renderer::<App>::new().render(); }
