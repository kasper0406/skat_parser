import css from './style.css';
import recordspec from './record-spec.yml';
import errorspec from './error-spec.yml';

const rust = import("./pkg");

fetch(recordspec)
    .then(response => response.text())
    .then(recordSpecString => {
        fetch(errorspec)
            .then(response => response.text())
            .then(errorSpecString => {
                rust.then(rust => {
                    console.log("Initializing rust code");
                    rust.run(recordSpecString, errorSpecString);
                });
            })
    });
