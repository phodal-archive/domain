// import("../pkg/index.js").catch(console.error);

require.ensure([], function () {
    const rust = require("../pkg");
    rust.helo("hello, world!")
});
