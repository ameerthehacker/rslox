import './App.css';
import 'h8k-components';

import React, { useState } from 'react';
const title = "React App";

const App = () => {
    const [count, setCount] = useState(0);

    return (
        <div className="App">
            <h8k-navbar header={title}></h8k-navbar>

            <div className="fill-height layout-column justify-content-center align-items-center">
                <p data-testid="output">You clicked {count} times ...</p>
                <button data-testid="add-button" onClick={() => setCount(count + 1)}>Click Me</button>
            </div>
        </div>
    );
}

export default App;
