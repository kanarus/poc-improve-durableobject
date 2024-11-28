use worker::{Request, Response, Env, Context, Result, DurableObject};

#[worker::event(fetch)]
pub async fn main(_: Request, env: Env, _: Context) -> Result<Response> {
    env.durable_object("POC")?
        .id_from_name("global")?
        .get_stub()?
        .fetch_with_str("http://poc")
        .await
}

#[DurableObject(fetch)]
struct POC {
    state: worker::State,
}

impl DurableObject for POC {
    fn new(state: worker::State, _: Env) -> Self {
        Self { state }
    }

    async fn fetch(&mut self, _: Request) -> Result<Response> {
        let id = self.state.id();
        Response::ok(format!("{id}: The concept is proven."))
    }
}

#[DurableObject(fetch)]
struct POC2; // doesn't cause compile error

impl DurableObject for POC2 {
    fn new(_: worker::State, _: Env) -> Self {
        todo!()
    }
    async fn fetch(&mut self, _: Request) -> Result<Response> {
        todo!()
    }
}
