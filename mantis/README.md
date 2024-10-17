# Mantis

Mantis is a Rust crate for building and training machine learning models using federated learning. It leverages Hugging Face's Candle framework to provide a flexible and efficient way to train models on decentralized data.

## Features

- **Federated Learning**: Mantis supports both centralized and federated training paradigms, allowing you to choose the right approach for your use case.
- **Hugging Face Integration**: Utilizes Hugging Face's Candle framework for easy integration with popular machine learning models and datasets.
- **Customizable**: Provides a modular architecture that allows you to customize the training process to your specific needs.
- **Data Privacy**: Provides a data privacy module that allows you to train your model with data privacy guarantees.
- **Built for the Edge**: Enables efficient, multitasking applications on embedded systems.

## Getting Started

To get started with Mantis, add the following to your `Cargo.toml` file:

```toml
[dependencies]
mantis = "0.1.0"
```

