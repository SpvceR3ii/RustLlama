# How to setup RustLlama
## Step 1: Set up the application
1. Download the RustLlama ZIP file, and extract it.
2. Enter the directory: `cd RustLlama`
3. Set up the Rust build: `cargo build --release` *(REQUIRES RUST & CARGO TO BE INSTALLED!)*
4. Enter the release build `cd target/release`
5. Initialise the config directory, either by copying the example configuration from the RustLlama directory, or making a new one. `cp -r ~/RustLlama/config ./`
6. Modify the existing config file:
```
{
    "model_name": "qwen2.5:3b",
    "do_streaming": true,
    "system_prompt": "You are an AI assistant. You must help user and assist them."
}
```
- `model_name`: The AI models name, e.g llama3.2:3b (Must be pulled via Ollama, read further to find out how to pull models.)
- `do_streaming`: Sends processed data once collected instead of waiting for the whole batch of data. Streaming is enabled by default.
- `system_prompt`: Sets the AI's tone and manner. e.g, "You are Eminem, a rapper and musician."

## How to pull models from Ollama
1. Get a model. This model can be selected from [The Ollama Library](https://ollama.com/library) and contains various models, such as LLaMa, Gemma, and Qwen.
2. Open the Ollama CLI. Enter your CMD/Terminal and type `ollama pull [model name]` or `ollama run [model name]`. This will run the Ollama Chatting interface. 

To run a model with RustLlama, follow the steps above to continue. Enjoy!

> [!NOTE]
> This application is still in a pre-beta state, and can break at any moment. Please open an issue ticket for any bugs you encounter.