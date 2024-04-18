/**
 * Different types of supporte input types.
 * @typedef {"text"|"password"|"textarea"} InputType
 */

/**
 * The metadata describing an input.
 * @typedef {object} Input
 * @property {string} inputName The `name` and `id` property of the input.
 * @property {InputType} inputType The `type` property of the input.
 * @property {string|null} placeholder The `placeholder` property of the input. (Optional.)
 */

/**
 * What will be displayed to the user.
 * @typedef {object} DialogInfo
 * @property {string} prompt The heading of the dialog.
 * @property {string} promptIcon The icon next to the heading.
 */

/**
 * @type {Element} The dialog template.
 */
const DIALOG_TEMPLATE = document.querySelector("template#dialog-template")
if (DIALOG_TEMPLATE === null) {
    console.error("CANNOT FIND DIALOG TEMPLATE!")
}

/**
 * @type {Element} The dialog template.
 */
const DIALOGS_GOES_HERE = document.querySelector(".dialogs-go-here")
if (DIALOGS_GOES_HERE === null) {
    console.error("CANNOT FIND PLACE TO PUT DIALOGS!")
}
// DIALOGS_GOES_HERE.attachShadow({ mode: "open" })

/**
 * Automatically:
 *      creates a dialog with a form,
 *      inserts it into `<div class="dialogs-go-here"></div>`,
 *      and hooks it up to send a request.
 * @param {string} dialogId The `data-dialog-button` value of the button to hook up the dialog to.
 * @param {DialogInfo} dialogInfo What will be displayed to the user.
 * @param {Input[]} inputs A map of the inputs to pass to the request.
 */
function createDialog(dialogId, dialogInfo, inputs) {
    const buttonElement = document.querySelector(`[data-dialog-button="${dialogId}"]`)
    if (!buttonElement) {
        console.dir(dialogId, { depth: null })
        throw new Error("That button doesn't exist!")
    }

    const method = buttonElement.getAttribute("data-dialog-method")
    const action = buttonElement.getAttribute("data-dialog-action")

    if (!method || !action) {
        console.dir(dialogId, { depth: null })
        throw new Error("Button doesn't specify both `data-dialog-method` and `data-dialog-target`!")
    }

    /**
     * @type {HTMLTemplateElement}
     */
    let dialog = DIALOG_TEMPLATE.content.cloneNode(true).querySelector("dialog")
    console.dir(dialog, { depth: null })
    dialog.setAttribute("data-dialog-box", dialogId)

    /**
     * @type {HTMLFormElement}
     */
    let form = dialog.querySelector("form")
    form.method = method
    form.action = action

    let prompt = document.createElement("h2")
    prompt.slot = "prompt"
    prompt.innerText = dialogInfo.prompt
    prompt.setAttribute("icon", dialogInfo.promptIcon)

    dialog.appendChild(prompt)

    let inputElements = inputs.map(input => {
        let inputElement = input.inputType === "textarea" ? document.createElement("textarea") : document.createElement("input")

        if (input.inputType !== "textarea")
            inputElement.type = input.inputType

        inputElement.placeholder = input.placeholder
        inputElement.name = input.inputName
        inputElement.id = input.inputName

        return inputElement
    })

    for (const inputElement of inputElements) {
        form.appendChild(inputElement)
    }

    buttonElement.addEventListener("click", () => dialog.showModal())

    const closeButton = dialog.querySelector(".btn.close-dialog")
    closeButton.addEventListener("click", event => {
        event.preventDefault()
        dialog.close()
    })

    dialog.appendChild(form)
    DIALOGS_GOES_HERE.appendChild(dialog)
}

createDialog(
    "create-user",
    {
        prompt: "Skapa användare",
        promptIcon: "person_add"
    },
    [
        { inputName: "username", inputType: "text", placeholder: "sven.svensson" },
        { inputName: "password", inputType: "password", placeholder: "superSecret45" }
    ]
)

const dialogButtons = document.querySelectorAll("[data-dialog-button]")

dialogButtons.forEach(dialogButton => {
    const dialogId = dialogButton.getAttribute("data-dialog-button")
    const dialog = document.querySelector(`[data-dialog-box="${dialogId}"]`)
    if (!dialog) return "No dialog."
    const closeButton = dialog.querySelector(".btn.close-dialog")

    dialogButton.addEventListener("click", () => dialog.showModal())
    closeButton.addEventListener("click", event => {
        event.preventDefault()
        dialog.close()
    })
})
