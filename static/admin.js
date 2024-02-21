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
}
