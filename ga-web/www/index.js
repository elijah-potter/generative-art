import * as wasm from "generative-art";

wasm.set_panic_hook();

var selector = document.getElementById("generator");
selector.onchange = select_generator;

var celestial_group = document.getElementById("celestial");

function select_generator() {
    celestial_group.style.display = "none";

    switch (selector.value) {
        case "celestial":
            celestial_group.style.display = "initial";
            celestial_page();
            break;
        case "waves":
            break;
    }
}

select_generator();

function celestial_page() {
    var render_count = document.getElementById("render_count");
    render_count.onchange = render_canvas;
    render_count.oninput = render_canvas;

    var object_count = document.getElementById("object_count");
    object_count.onchange = object_count_change;
    object_count.oninput = object_count_change;

    function object_count_change() {
        render_count.max = this.value;
        render_count.value = this.value;
        render_canvas();
    }

    var min_object_size = document.getElementById("min_object_size");
    min_object_size.onchange = render_canvas;
    min_object_size.oninput = render_canvas;

    var max_object_size = document.getElementById("max_object_size")
    max_object_size.onchange = max_object_size_change;
    max_object_size.oninput = max_object_size_change;

    function max_object_size_change() {
        min_object_size.max = this.value;
        min_object_size.value = Math.min(min_object_size.value, this.value);
        render_canvas();
    }

    var g = document.getElementById("g");
    g.onchange = render_canvas;
    g.oninput = render_canvas;

    var steps = document.getElementById("steps");
    steps.onchange = render_canvas;
    steps.oninput = render_canvas;

    var step_length = document.getElementById("step_length");
    step_length.onchange = render_canvas;
    step_length.oninput = render_canvas;

    var zoom = document.getElementById("zoom");
    zoom.onchange = render_canvas;
    zoom.oninput = render_canvas;

    var seed = document.getElementById("seed");
    seed.onchange = render_canvas;
    seed.oninput = render_canvas;

    var randomize = document.getElementById("randomize");
    randomize.onclick = function () {
        seed.value = Math.floor(Math.random() * 10000000);
        render_canvas();
    }
    randomize.onclick();

    var svg_download = document.getElementById("svg_download");
    svg_download.onclick = function () {
        var svg = wasm.celestial(object_count.value,
            render_count.value,
            min_object_size.value,
            max_object_size.value,
            g.value,
            steps.value,
            step_length.value,
            zoom.value,
            Math.floor(seed.value),
            2);

        download_blob(svg, "celestial.svg", "image/svg+xml");
    }

    var png_download = document.getElementById("png_download");
    png_download.onclick = function () {
        var png = wasm.celestial(object_count.value,
            render_count.value,
            min_object_size.value,
            max_object_size.value,
            g.value,
            steps.value,
            step_length.value,
            zoom.value,
            Math.floor(seed.value),
            3);

        download_blob(png, "celestial.png", "image/png");
    }

    function render_canvas() {
        wasm.celestial(object_count.value,
            render_count.value,
            min_object_size.value,
            max_object_size.value,
            g.value,
            steps.value,
            step_length.value,
            zoom.value,
            Math.floor(seed.value),
            1);
    }
}

function download_url(data, fileName) {
    const a = document.createElement('a')
    a.href = data
    a.download = fileName
    document.body.appendChild(a)
    a.style.display = 'none'
    a.click()
    a.remove()
}

function download_blob(data, fileName, mimeType) {

    const blob = new Blob([data], {
        type: mimeType
    })

    const url = window.URL.createObjectURL(blob)

    download_url(url, fileName)

    setTimeout(() => window.URL.revokeObjectURL(url), 1000)
}