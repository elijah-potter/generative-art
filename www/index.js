import * as wasm from "generative-art-wasm";

const upload = document.getElementById('upload');

upload.addEventListener('change', (event) => {
    const fileList = event.target.files;

    const reader = new FileReader();
    reader.addEventListener('load', (event) => {
        console.log("Loaded");

        document.getElementById("before").src = event.target.result;
        
        document.getElementById("after").src = wasm.sketch_file(event.target.result, fileList[0].name, 5000);
    });

    reader.readAsDataURL(fileList[0]);
});