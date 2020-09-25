import css from './style.css';
import recordspec from './record-spec.yml';

const rust = import("./pkg");

console.log("Record spec: ", recordspec);

fetch(recordspec)
    .then(response => response.text())
    .then(recordSpecString => {
        rust.then(rust => {
            console.log("Initializing rust code");
            rust.run(recordSpecString);
        });
    });
