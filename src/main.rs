//! легендарный ананасовый бот теперь на расте (omg)
//! подписывайтесь ставьте лайки

use lazy_static::lazy_static;
use markov::Chain;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};
use teloxide::{prelude2::*, utils::command::BotCommand};

lazy_static! {
    pub static ref CHAIN: Arc<Mutex<Chain<String>>> = {
        let chain = Chain::load("chain");
        let chain = match chain {
            Ok(res) => res,
            Err(_) => {
                println!("бля цепи нет либо она нахуй сломалась, ща новую сделаю ок");
                let c = Chain::new();
                c.save("chain").unwrap();
                c
            }
        };
        Arc::new(Mutex::new(chain))
    };
}

pub type BotResult = Result<(), Box<dyn Error + Sync + Send>>;

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "ну вот короч:")]
pub enum Command {
    #[command(description = "ого хелп")]
    Help,
    #[command(description = "высрать текст ебать)))))))))))")]
    Gen,
}

/// сборник фильтров
mod filters {
    use crate::*;

    /// пропускает только если находится в ананасах, либо тестовом чате
    pub fn group_only_filter(m: Message) -> bool {
        vec![-1001444484622, -1001197098429].contains(&m.chat.id)
    }

    /// пропускает только если есть текст
    pub fn contains_text_filter(m: Message) -> bool {
        m.text().is_some()
    }

    /// пропускает только если длина сообщения менее 150
    pub fn text_length_filter(m: Message) -> bool {
        m.text().unwrap().chars().count() < 150
    }
}

/// сборник команд
mod commands {
    use crate::*;

    /// угадай бля
    pub async fn help_command(m: Message, bot: AutoSend<Bot>) -> BotResult {
        bot.send_message(m.chat.id, Command::descriptions()).await?;

        Ok(())
    }

    /// генерит пасту
    pub async fn gen_command(m: Message, bot: AutoSend<Bot>) -> BotResult {
        let text = crate::CHAIN.lock().unwrap().generate_str();
        bot.send_message(m.chat.id, text).await?;

        Ok(())
    }
}

/// сборник обработчиков приколов
mod handlers {
    use crate::*;

    /// сбор паст сообщений (omg майкрософт телеметрия)
    pub async fn collect_messages(m: Message, _: AutoSend<Bot>) -> BotResult {
        CHAIN
            .lock()
            .unwrap()
            .feed_str(&m.text().unwrap().to_ascii_lowercase().trim());
        CHAIN.lock().unwrap().save("chain")?;

        Ok(())
    }

    /// расфасовывает команды по нужным методам
    pub async fn handle_commands(m: Message, bot: AutoSend<Bot>, command: Command) -> BotResult {
        match command {
            Command::Help => commands::help_command(m, bot).await?,
            Command::Gen => commands::gen_command(m, bot).await?,
        };

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bot = Bot::from_env().auto_send();

    let handler = dptree::entry().branch(
        Update::filter_message()
            .branch(
                dptree::entry()
                    .filter_command::<Command>()
                    .endpoint(handlers::handle_commands),
            )
            .branch(
                dptree::entry()
                    .chain(dptree::filter(filters::group_only_filter))
                    .chain(dptree::filter(filters::contains_text_filter))
                    .chain(dptree::filter(filters::text_length_filter))
                    .endpoint(handlers::collect_messages),
            ),
    );

    bot.set_my_commands(Command::bot_commands()).await?;

    Dispatcher::builder(bot, handler)
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;

    Ok(())
}
