/**
 * Rigs up all `block-actions` on a block.
 * @param {HTMLElement} block The block to rig.
 */
function rigBlock(block) {
    const moveUpButton = block.querySelector("[icon=\"keyboard_arrow_up\"]")
    const moveDownButton = block.querySelector("[icon=\"keyboard_arrow_down\"]")
    const deleteButton = block.querySelector("[icon=\"delete\"]")
    const addButton = block.querySelector("[icon=\"add\"]")

    deleteButton.addEventListener("click", () => block.remove())
    moveUpButton.addEventListener("click", () => {
        const prevBlock = block.previousElementSibling
        if (prevBlock) {
            blockEditor.insertBefore(block, prevBlock)
        }
    })
    moveDownButton.addEventListener("click", () => {
        const nextBlock = block.nextElementSibling
        if (nextBlock) {
            blockEditor.insertBefore(nextBlock, block)
        }
    })
    addButton.addEventListener("click", () => {
        alert("Not implemented yet!")
    })

}

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
    "YouTube": ["video_id", "caption"],
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
const BLOCK_TEMPLATE = `<div class="block" data-block-type="__BLOCK_TYPE__">
    __BLOCK_DATA__
    <div class="block-actions">
        <button class="btn icon-only" type="button" icon="keyboard_arrow_up" title="Flytta blocket uppåt"></button>
        <button class="btn icon-only" type="button" icon="keyboard_arrow_down" title="Flytta blocket nedåt"></button>
        <div class="sep"></div>
        <button class="btn dangerous icon-only" type="button" icon="delete" title="Radera blocket"></button>
        <button class="btn icon-only" type="button" icon="add" title="Lägg till block under"></button>
    </div>
</div>`

const BLOCK_TEMPLATES = Object.freeze({
    "Paragraph": `
        <textarea class="body_text" placeholder="Skriv brödtext här..."></textarea>
    `,
    "Image": `
        <p>Bildens ID:</p>
        <input class="id" value="{{ block.id }}" placeholder="Skriv bildens id här">
        <p>Bildtext:</p>
        <input class="caption" value="{{ block.caption }}" placeholder="Skriv bildtext här...">
    `,
    "Quote": `
        <label for="quote">Citat:</label>
        <input class="quote" value="{{ block.quote }}" placeholder="Skriv citat här...">
        <label for="citation">Referens:</label>
        <input class="citation" value="{{ block.citation }}" placeholder="Skriv vem som sa/skrev det här...">
    `,
    "Heading": `
        <input class="heading" placeholder="Skriv rubrik här...">
    `,
    "RawHtml": `
        <textarea class="html" placeholder="Bädda in din HTML här."></textarea>
    `,
    "YouTube": `
        <p>YouTube-videons ID:</p>
        <input class="video_id" placeholder="Skriv YouTube-videons id här">
        <p>Bildtext:</p>
        <input class="caption" placeholder="Skriv bildtext här...">
    `,
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

    const block = BLOCK_TEMPLATE.replace("__BLOCK_TYPE__", blockType).replace("__BLOCK_DATA__", blockTemplate)
    blockEditor.insertAdjacentHTML("beforeend", block)
    rigBlock(blockEditor.lastChild)
    addBlockDialog.close()
})

// Automatically rescale textareas to fit the text
const textAreas = document.querySelectorAll("textarea")
textAreas.forEach(textArea => {
    textArea.style.height = textArea.scrollHeight + "px"
    textArea.addEventListener("input", () => {
        textArea.style.height = textArea.scrollHeight + "px"
    })
})

const blocks = document.querySelectorAll(".block")
blocks.forEach(rigBlock)
