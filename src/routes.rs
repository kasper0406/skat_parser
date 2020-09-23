use yew_router::{prelude::*, Switch};

#[derive(Debug, Switch, Clone)]
pub enum EindkomstRoutes {
    #[to = "/"]
    Welcome,

    #[to = "/eindkomst"]
    Eindkomst,
}
