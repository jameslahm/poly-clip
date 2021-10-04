import * as wasm from "hello-wasm-pack";
import React, { StrictMode } from 'react'
import ReactDOM from 'react-dom'
import App from './App'

wasm.greet();

ReactDOM.render(<StrictMode><App /></StrictMode>, document.getElementById("root"))
