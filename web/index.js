import init from 'pkg';

init().then(() => {
    console.log("loaded");
}).catch((error) => {
    if (!error.message.startsWith("Using exceptions for control flow,")) {
        throw error;
    }
});