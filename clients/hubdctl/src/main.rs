#![doc(html_favicon_url = "https://qaul.org/favicon.ico")]
#![doc(html_logo_url = "https://qaul.org/img/qaul_icon-128.png")]

use futures::join;
use libqaul::{error::Result, helpers::TagSet, messages::{Mode, IdType}, Qaul};
use ratman::Router;

#[async_std::main]
async fn main() -> Result<()> {
    let r = Router::new();
    // TDOD: Add network drivers

    let q = Qaul::new(r);
    let user = q.users().create("password").await?;

    let msg = q.messages();
    let send = msg.send(
        user.clone(),
        Mode::Flood,
        IdType::unique(),
        "de.spacekookie.myapp",
        TagSet::empty(),
        vec![1, 2, 3, 4],
    );
    let subscriber = msg
        .subscribe(user, "de.spacekookie.myapp", TagSet::empty())
        .await?;

    join!(send, subscriber.next(),).0.unwrap();
    Ok(())
}
