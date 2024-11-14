/**
 * @constant {Object} BLOCK_TYPE_TO_FIELDS
 * Maps block types to field names for that block type.
 */
const BLOCK_TYPE_TO_FIELDS = Object.freeze({
    "Paragraph": ["body_text"],
    "Image": ["id", "caption"],
    "Quote": ["quote", "citation"],
    "Heading": ["heading"],
    "RawHtml": ["html"],
})

const textForm = document.querySelector("#text-form")
textForm.addEventListener(
    "submit",
    event => {
        event.preventDefault()
        let formData = new FormData(textForm)

        const blockDivs = Array.from(textForm.querySelectorAll(".block-editor > .block"))

        const blocks = blockDivs.map(blockDiv => {
            const blockType = blockDiv.getAttribute("data-block-type")
            if (!BLOCK_TYPE_TO_FIELDS[blockType]) {
                alert(`Unknown block type: ${blockType}`)
                throw new Error(`Unknown block type: ${blockType}`)
            }

            let blockData = {}
            BLOCK_TYPE_TO_FIELDS[blockType].forEach(fieldName => {
                const fieldData = blockDiv.querySelector(`.${fieldName}`).value
                blockData[fieldName] = fieldData
            })

            blockData.type = blockType

            return blockData
        })

        let textData = {}

        for (const key of formData.keys()) {
            textData[key] = formData.get(key)
        }

        textData["text-id"] = Number(textData["text-id"])

        textData.blocks = blocks

        fetch(
            textForm.action,
            {
                method: textForm.method,
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify(textData)
            }
        ).then(
            result => result.json()
        ).then(
            response => window.location.replace(response.redirect)
        ).catch(
            error => {
                alert("Encountered an error! Check the console for more information.")
                console.error(error)
            }
        )
    }
)

const BLOCK_TEMPLATES = Object.freeze({
    "Paragraph": `<div class="block" data-block-type="Paragraph">
        <textarea class="body_text" placeholder="Skriv brödtext här..."></textarea>
    </div>`,
    "Image": `<div class="block" data-block-type="Image">
        <input class="id" placeholder="Skriv bildens id här">
        <input class="caption" placeholder="Skriv bildtext här...">
    </div>`,
    "Quote": `<div class="block" data-block-type="Quote">
        <input class="quote" placeholder="Skriv citat här...">
        <input class="citation" placeholder="Skriv vem som sa/skrev det här...">
    </div>`,
    "Heading": `<div class="block" data-block-type="Heading">
        <input class="heading" placeholder="Skriv rubrik här...">
    </div>`,
    "RawHtml": `<div class="block" data-block-type="RawHtml">
        <textarea class="html" placeholder="Bädda in din HTML här."></textarea>
    </div>`,
})

const addBlockButton = document.querySelector("button.add-block")
const addBlockDialog = document.querySelector("dialog.add-block")
const closeButton = addBlockDialog.querySelector("button.close-dialog")
const blockEditor = document.querySelector(".block-editor")
addBlockButton.addEventListener("click", () => addBlockDialog.showModal())
closeButton.addEventListener("click", event => {
    event.preventDefault()
    addBlockDialog.close()
})
addBlockDialog.addEventListener("submit", event => {
    event.preventDefault()
    const blockType = addBlockDialog.querySelector("select[name=block-type]").value
    const blockTemplate = BLOCK_TEMPLATES[blockType]
    blockEditor.insertAdjacentHTML("beforeend", blockTemplate)
    addBlockDialog.close()
})
