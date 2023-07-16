const view = document.getElementById("button-addon2");
view.addEventListener('click', () => {
    const mySelect = document.getElementById("mySelect");
    const selectedOption = mySelect.options[mySelect.selectedIndex].value;
    const fluid = document.getElementById("fluid").value;
    console.log(fluid);
    import("./node_modules/trial/trial.js").then((js) => {
        js.start().catch(console.error);
    });
});