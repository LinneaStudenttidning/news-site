
const dialogs = document.querySelectorAll('.show-dialog');

dialogs.forEach(el => {
    if(el.id) {
        console.log(`dialog-${el.id}`)
        const dialog = document.getElementById(`dialog-${el.id}`);
        el.addEventListener('click', event => {
            dialog.showModal();
        })
        const closeDialogButton = document.getElementById(`close-${el.id}`);
        closeDialogButton.addEventListener("click", (e) => {
            e.preventDefault();
            dialog.close();
        });
    }
});

