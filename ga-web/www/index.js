import * as wasm from "generative-art";

wasm.set_panic_hook();

document.getElementById("width").onchange = change;
document.getElementById("object_count").onchange = change;
document.getElementById("height").onchange = change;
document.getElementById("min_object_size").onchange = change;
document.getElementById("max_object_size").onchange = change;
document.getElementById("min_object_velocity").onchange = change;
document.getElementById("max_object_velocity").onchange = change;
document.getElementById("g").onchange = change;
document.getElementById("dots").onchange = change;
document.getElementById("button").onclick = change;

document.getElementById("randomize").onclick = randomize_seed;

change();

function randomize_seed() {
    const seed = Math.floor(Math.random() * (18446744073709551615));
    document.getElementById("seed").value = seed;
    console.log("Set seed to: ", seed)
}

function change() {
    let settings = wasm.CelestialSketcherSettings.new();

    settings.width = document.getElementById("width").value;
    settings.height = document.getElementById("height").value;
    settings.object_count = document.getElementById("object_count").value;
    settings.render_count = document.getElementById("object_count").value;
    settings.min_object_size = document.getElementById("height").value;
    settings.max_object_size = document.getElementById("max_object_size").value;
    settings.min_object_size = document.getElementById("min_object_size").value;
    settings.min_object_velocity = document.getElementById("min_object_velocity").value;
    settings.max_object_velocity = document.getElementById("max_object_velocity").value;
    settings.g = document.getElementById("g").value;
    settings.dots = document.getElementById("dots").value;

    console.log(settings);

    document.getElementById("image").setAttribute("src", wasm.celestial(settings, document.getElementById("seed").value))
}
