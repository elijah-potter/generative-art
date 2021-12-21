import * as wasm from "generative-art";

wasm.set_panic_hook();

celestial_page();

function celestial_page() {
    var render_count = document.getElementById("render_count");
    render_count.onchange = render;
    render_count.oninput = render;

    var object_count = document.getElementById("object_count");
    object_count.onchange = object_count_change;
    object_count.oninput = object_count_change;

    function object_count_change() {
        render_count.max = this.value;
        render_count.value = this.value;
        render();
    }

    var min_object_size = document.getElementById("min_object_size");
    min_object_size.onchange = render;
    min_object_size.oninput = render;

    var max_object_size = document.getElementById("max_object_size")
    max_object_size.onchange = max_object_size_change;
    max_object_size.oninput = max_object_size_change;

    function max_object_size_change() {
        min_object_size.max = this.value;
        min_object_size.value = Math.min(min_object_size.value, this.value);
        render();
    }

    var g = document.getElementById("g");
    g.onchange = render;
    g.oninput = render;

    var steps = document.getElementById("steps");
    steps.onchange = render;
    steps.oninput = render;

    var step_length = document.getElementById("step_length");
    step_length.onchange = render;
    step_length.oninput = render;

    var zoom = document.getElementById("zoom");
    zoom.onchange = render;
    zoom.oninput = render;

    function render() {
        console.log(
            object_count.value,
            render_count.value,
            min_object_size.value,
            max_object_size.value,
            g.value,
            steps.value,
            step_length.value,
            zoom.value);

        wasm.celestial(object_count.value,
            render_count.value,
            min_object_size.value,
            max_object_size.value,
            g.value,
            steps.value,
            step_length.value,
            zoom.value);
    }

    render();
}