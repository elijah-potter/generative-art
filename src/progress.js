export function progress(i) {
    const progress_bar = document.getElementById("progress");
    console.log(i);
    console.log(progress_bar);
    progress_bar.value = i;
    setTimeout(function () {
        document.getElementById("pb").value=this;
        if (this == max) console.log("Finished")
      }.bind(i+1), i*16);}
