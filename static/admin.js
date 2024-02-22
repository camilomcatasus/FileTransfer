
window.onload = () => {
     document.addEventListener("focusin", (event) => {
        if(event.target.matches("input[password]"))
        {
            event.target.type = "text";
        }
    });

    document.addEventListener("focusout", (event) => {
        if(event.target.matches("input[password]"))
        {
            event.target.type = "password";
        }
    });

    document.addEventListener("alerterror", (event) => {
        var elemDiv = document.createElement("div");
        elemDiv.innerText = event.detail.value;
        elemDiv.className = "w-[400px] rounded p-3 bg-red-300 text-red-900";

        document.getElementById("alert-container").appendChild(elemDiv);
        console.log(event);
    });
    document.addEventListener("alertsuccess", (event) => {
        var elemDiv = document.createElement("div");
        elemDiv.innerText = event.detail.value;
        elemDiv.className = `w-[400px] rounded p-3 
        bg-green-300 text-green-900 border-green-900 
        drop-shadow flex border transition-opacity opacity-100 duration-[3000ms]
        `;
        document.getElementById("alert-container").appendChild(elemDiv);

        setTimeout(() => {
            elemDiv.classList.remove("opacity-100");
            elemDiv.classList.add("opacity-0");
        }, 3000);
        console.log(event);
    });

    document.addEventListener("transitionend", (event) => {
        if(event.target.matches("#alert-container > div"))
        {
            event.target.remove();
        }
    });
}

