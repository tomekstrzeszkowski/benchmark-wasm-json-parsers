import React, {Component} from 'react';
//import './App.css';
import * as ReactDOM from 'react-dom'
import TextareaAutosize from '@material-ui/core/TextareaAutosize';
import Button from '@material-ui/core/Button';
import * as wasmRust from "wasm-rust";

declare global {
    interface Window {
        parseJson:any;  // wasm function will be incuded independly
    }
}
 
class App extends Component<{}, { value: string, result: number, parsed: string }> {
  constructor(props: any) {
    super(props);
    this.state = {value: "", result: 0, parsed: ""};

    this.handleChange = this.handleChange.bind(this);
    this.handleTimeItGo = this.handleTimeItGo.bind(this);
    this.handleTimeItRust = this.handleTimeItRust.bind(this);
    this.loadServerData = this.loadServerData.bind(this);
    this.loadBrowserData = this.loadBrowserData.bind(this);
    this.loadBrowserData();
  }

  async loadServerData() {
    const response: any = await fetch('cars.json');
    const cars: any = await response.json()
    this.setState({ value: JSON.stringify(cars), parsed: "", result: 0 });
  }  

  loadBrowserData() {
    this.setState({ 
      value: '[{"Name":"chevy s-10","Miles_per_Gallon":31,"Cylinders":4,"Displacement":"119","Horsepower":82,"Weight_in_lbs":2720,"Acceleration":19.4,"Year":"1982-01-01","Origin":"USA"}]',
      parsed: "",
      result: 0
    });
  }

  handleChange(event: any) {
    this.setState({value: event.target.value});
  }

  handleTimeItGo(event: any) {
    const start = performance.now();
    const parsed: string = window.parseJson(this.state.value);
    const end = performance.now();
    const duration = end - start;
    this.setState({result: duration, parsed: parsed});
    event.preventDefault();
  }
  handleTimeItRust() {
    wasmRust.greet();
  }

  render(){
    return (
      <div style={{ display: 'flex', flexDirection: 'column', justifyContent: 'center' }}>
        <div style={{ display: 'flex', justifyContent: 'space-evenly' }}>
          <div>
            <Button onClick={this.handleTimeItGo} variant="contained">Time It GO-lang!</Button>
            <Button onClick={this.handleTimeItRust} variant="contained">Time It Rust!</Button>
          </div>
          <div style={{ display: 'flex', justifyContent: 'left' }}>
            <Button onClick={this.loadBrowserData} variant="contained" color="secondary">load browser data</Button>
            <Button onClick={this.loadServerData} variant="contained" color="secondary">load server data</Button>
          </div>
        </div>
        { this.state.result ? (
          <div style={{ display: 'flex', justifyContent: 'center' }}>
            <div>
              <label>WASM method execution time:</label>
            </div>
            <div>
              {this.state.result} [ms]
            </div>
          </div>
          ) : <div></div>
        }
        { this.state.result ? (
          <div style={{ display: 'flex', justifyContent: 'center' }}>
           <label>Parsed result:</label>
           <code>{this.state.parsed}</code>
          </div>
          ): <div></div>
        }
        <div style={{ display: 'flex', justifyContent: 'center' }}>
          <form>
            <label>
              <TextareaAutosize 
                value={this.state.value} 
                onChange={this.handleChange}
                minRows={3}
                style={{ width: "100%" }}
              />
            </label>
          </form>
        </div>
      </div>
    );
  }
  
}
 
ReactDOM.render(<App />, document.getElementById('root'));