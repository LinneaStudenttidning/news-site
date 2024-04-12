const dialogButtons = document.querySelectorAll("[data-dialog-button]")

dialogButtons.forEach(dialogButton => {
    const dialogId = dialogButton.getAttribute("data-dialog-button")
    const dialog = document.querySelector(`[data-dialog-box="${dialogId}"]`)
    const closeButton = dialog.querySelector(".btn.close-dialog")

    dialogButton.addEventListener("click", () => dialog.showModal())
    closeButton.addEventListener("click", event => {
        event.preventDefault()
        dialog.close()
    })
})

const shareButtons = document.querySelectorAll(".share")

shareButtons.forEach(shareButton => {
    if (shareButton.attributes["data-url"]) {
        const url = shareButton.getAttribute("data-url")
        const title = shareButton.getAttribute("data-title")

        shareButton.addEventListener("click", () => {
            if (typeof navigator.share === "function") {
                navigator.share({ "url": url, "title": title })
            }
            else {
                navigator.clipboard.writeText(url)
                alert("Copied link to clipboard!")
            }

        })
    }
})
