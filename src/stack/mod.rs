pub mod parser;

use crate::localstack;
use parser::Stack;

pub async fn up(stack: Stack) -> Result<(), Box<dyn std::error::Error>> {
    match localstack::is_running().await {
        Ok(true) => {
            match localstack::running_version().await {
                Ok(running_version) => if running_version == stack.localstack_version {
                    println!("TODO:// deploy the stack");
                    // deploy(stack);
                } else {
                    localstack::stop().await?;
                    localstack::remove().await?;
                    localstack::start(&stack.localstack_version).await?;

                    println!("TODO:// deploy the stack");

                    // TODO: use recursion `up(stack).await?`
                    // so that we can call deploy(stack) only in one palce which is conditioned by
                    // by if the running_version is same as the version specified in stack.localstack_version
                    //
                    // deploy(stack);
                },
                Err(_) => println!("Error??"),
            }

        },

        Ok(false) => {
            localstack::remove().await?;
            localstack::start(&stack.localstack_version).await?;

            println!("TODO:// deploy the stack");

            // TODO: use recursion `up(stack).await?`
            // so that we can call deploy(stack) only in one palce which is conditioned by
            // by if the running_version is same version specified in stack.localstack_version
            //
            // deploy(stack);
        },

        Err(_) => println!("Error??"),
    }

    Ok(())
}
