import * as wasm from "generative-art";

wasm.set_panic_hook();

var selector = document.getElementById("generator");
selector.onchange = select_generator;

var celestial_group = document.getElementById("celestial");
var waves_group = document.getElementById("waves");

function select_generator() {
    celestial_group.style.display = "none";
    waves_group.style.display = "none";

    switch (selector.value) {
        case "celestial":
            celestial_group.style.display = "initial";
            celestial_page();
            break;
        case "waves":
            waves_group.style.display = "initial";
            waves_page();
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

function waves_page() {
    var image = null;
    var filename = null;
    
    var upload = document.getElementById("upload");
    upload.addEventListener('change', (event) => {
        const file_list = event.target.files;

        const reader = new FileReader();
        reader.addEventListener('load', (event) => {
            console.log("Loaded file");
            image = new Uint8Array(event.target.result);
            filename = upload.value.split(/(\\|\/)/g).pop().split('.').pop();
            wasm.load_image(image,filename);
            render_canvas();
        });

        reader.readAsArrayBuffer(file_list[0]);
    });

    var stroke_color = document.getElementById("stroke_color");
    stroke_color.value = "#FFFFFF";
    stroke_color.onchange = render_canvas;

    var background_color = document.getElementById("background_color");
    background_color.value = "#000000";
    background_color.onchange = render_canvas;

    var stroke_width = document.getElementById("stroke_width");
    stroke_width.onchange = render_canvas;

    var skip_rows = document.getElementById("skip_rows");
    skip_rows.onchange = render_canvas;

    var frequency_multiplier = document.getElementById("frequency_multiplier");
    frequency_multiplier.onchange = render_canvas;

    var amplitude_multiplier = document.getElementById("amplitude_multiplier");
    amplitude_multiplier.onchange = render_canvas;

    var box_blur_radius = document.getElementById("box_blur_radius");
    box_blur_radius.onchange = render_canvas;

    var brightness_threshold = document.getElementById("brightness_threshold");
    brightness_threshold.onchange = render_canvas;

    var stroke_with_frequency = document.getElementById("stroke_with_frequency");
    stroke_with_frequency.onchange = render_canvas;

    var svg_download = document.getElementById("svg_download");
    svg_download.onclick = function () {
        var svg = wasm.waves(
            stroke_color.value,
            background_color.value,
            stroke_width.value,
            skip_rows.value,
            frequency_multiplier.value * frequency_multiplier.value,
            amplitude_multiplier.value,
            false,
            brightness_threshold.value,
            box_blur_radius.value,
            stroke_with_frequency.checked,
            2
        );

        download_blob(svg, "waves.svg", "image/svg+xml");
    }

    var png_download = document.getElementById("png_download");
    png_download.onclick = function () {
        var png = wasm.waves(
            stroke_color.value,
            background_color.value,
            stroke_width.value,
            skip_rows.value,
            frequency_multiplier.value * frequency_multiplier.value,
            amplitude_multiplier.value,
            false,
            brightness_threshold.value,
            box_blur_radius.value,
            stroke_with_frequency.checked,
            3
        );

        download_blob(png, "waves.png", "image/png");
    }

    function render_canvas() {
        if (image != null) {
            console.log(stroke_with_frequency.checked);
            wasm.waves(
                stroke_color.value,
                background_color.value,
                stroke_width.value,
                skip_rows.value,
                frequency_multiplier.value * frequency_multiplier.value,
                amplitude_multiplier.value,
                false,
                brightness_threshold.value,
                box_blur_radius.value,
                stroke_with_frequency.checked,
                1
            );
        }
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