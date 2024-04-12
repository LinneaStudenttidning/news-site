
const dialogs = document.querySelectorAll('.show-dialog');

dialogs.forEach(el => {
    if(el.id) {
        const dialog = document.getElementById(`dialog-${el.id}`);

        el.addEventListener('click', _event => {
            dialog.showModal();
        })
        const closeDialogButton = document.getElementById(`close-${el.id}`);
        closeDialogButton.addEventListener("click", (e) => {
            e.preventDefault();
            dialog.close();
        });
    }
});

const shareButtons = document.querySelectorAll('.share');

shareButtons.forEach(el => {
    if(el.attributes['data-url']) {
        const url = el.attributes['data-url'].value
        const title = el.attributes['data-title'].value

        el.addEventListener('click', _event => {
            if (typeof navigator.share === "function") {
                navigator.share({ "url": url, "title": title })
            }
            else {
                navigator.clipboard.writeText(url);
            }
            
        })
    }
});
