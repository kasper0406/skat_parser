const rust = import("./pkg");

rust.then(rust => {
    console.log("Initializing rust code");

    rust.run();
});
