# Benchmarking Edge ML

Machine learning on edge devices is young and consequently has many approaches and frameworks to solve the same types of problems.  In order to help clarify the state of the art, this repo is an experiment to run several types of models on different device types and powered by different frameworks.

## Outcomes

This repo makes reasonable assumptions about the end to end setup in order to semi-accurately measure real world performance.  The following metrics are of particular interest to this experiment:

* Client CPU usage
* Client Memory usage
* Client Power usage
* Model Time to update

## Methodology

This experiment consists of two main functional components:  a client test harness written in Rust and a small training pipeline written in Python.  There will be four main tests:

* PyTorch
* Tensorflow
* PyTorch via ONNX
* Tensorflow via ONNX

Each test will consist of three classification models, simulating different scenarios:

* Unstructured Text (URI classifications)
* Structure Text (DOM classifications)
* Binary (Image classifications)
