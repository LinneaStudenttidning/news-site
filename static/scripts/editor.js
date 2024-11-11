/* eslint-disable no-undef */

// eslint-disable-next-line no-unused-vars
const editor = new EditorJS({ 
    /** 
    * Id of Element that should contain the Editor 
    */ 
    holder: "editorjs", 

    /** 
    * Available Tools list. 
    * Pass Tool's class or Settings object for each Tool you want to use 
    */ 
    tools: {
        /**
        * Each Tool is a Plugin. Pass them via 'class' option with necessary settings {@link docs/tools.md}
        */
        header: {
            class: Header,
            config: {
                levels: [2],
                defaultLevel: 2,
                placeholder: "Header"
            },
            shortcut: "CMD+SHIFT+H"
        },

        /**
        * Or pass class directly without any configuration
        */
        list: {
            class: List,
            inlineToolbar: true,
            shortcut: "CMD+SHIFT+L"
        },

        quote: {
            class: Quote,
            inlineToolbar: true,
            config: {
                quotePlaceholder: "Enter a quote",
                captionPlaceholder: "Quote's author",
            },
            shortcut: "CMD+SHIFT+O"
        }
    }
})

function processForm(e) {
    editor.save().then((outputData) => {
        console.log("Article data: ", outputData)
        document.getElementById("text-body").value = JSON.stringify(outputData)
        return true
    }).catch((error) => {
        if(e.preventDefault)
            e.preventDefault()
        console.log("Saving failed: ", error)
        return false
    })
}

let form = document.getElementById("text-form")
if (form.attachEvent) {
    form.attachEvent("submit", processForm)
} else {
    form.addEventListener("submit", processForm)
}

// const editors = document.querySelectorAll(".editor")

// editors.forEach(editor => {
//     const actions = editor.querySelector(".actions")
//     const textbox = editor.querySelector(".textbox")

//     const charCount = editor.querySelector("#char-count")
//     const charCountNoWs = editor.querySelector("#char-count-no-ws")
//     const wordCount = editor.querySelector("#word-count")

//     function setupAction(htmlClass, commands) {
//         actions.querySelector(htmlClass)?.addEventListener("click", event => {
//             event.preventDefault()
//             textbox?.focus()
//             document.execCommand(...commands
//                 .map(command =>
//                     // If a command is a function, execute it.
//                     Object.prototype.toString.call(command) == "[object Function]"
//                         ? command()
//                         : command
//                 )
//             )
//         })
//     }

//     setupAction("[icon=\"undo\"]", ["undo"])
//     setupAction("[icon=\"redo\"]", ["redo"])
//     setupAction("[icon=\"format_clear\"]", ["removeFormar"])
//     setupAction("[icon=\"format_h1\"]", ["formatBlock", false, "<h1>"])
//     setupAction("[icon=\"format_h2\"]", ["formatBlock", false, "<h2>"])
//     setupAction("[icon=\"format_paragraph\"]", ["formatBlock", false, "<p>"])
//     setupAction("[icon=\"format_bold\"]", ["bold"])
//     setupAction("[icon=\"format_italic\"]", ["italic"])
//     setupAction("[icon=\"format_list_bulleted\"]", ["insertUnorderedList"])
//     setupAction("[icon=\"format_list_numbered\"]", ["insertOrderedList"])
//     setupAction("[icon=\"add_photo_alternate\"]", ["insertImage", false, () => prompt("Länk till bild?")])
//     setupAction("[icon=\"chat\"]", ["insertText", false, () => "– "])

//     document.execCommand("defaultParagraphSeparator", false, "p")

//     const keyMap = {
//         "CtrlB": ["bold"],
//         "CtrlI": ["italic"],
//     }

//     textbox.addEventListener("keyup", event => {
//         event.preventDefault()
//         event.stopPropagation()
//         event.stopImmediatePropagation()

//         const keyCombo = [
//             event.ctrlKey ? "Ctrl" : "",
//             event.altKey ? "Alt" : "",
//             event.shiftKey ? "Shift" : "",
//             event.key.toUpperCase()
//         ].join("")

//         if (keyMap[keyCombo]) {
//             document.execCommand(...keyMap[keyCombo]
//                 .map(command =>
//                     // If a command is a function, execute it.
//                     Object.prototype.toString.call(command) == "[object Function]"
//                         ? command()
//                         : command
//                 )
//             )
//         }

//         console.dir({ keyCombo }, { depth: null })
//     })

//     textbox.addEventListener("input", () => {
//         charCount.textContent = textbox.textContent.length
//         charCountNoWs.textContent = textbox.textContent.replace(/\s+/g, "").length
//         wordCount.textContent = textbox.textContent.split(/\s+/).filter(word => word !== "").length
//     })

//     textbox.dispatchEvent(new Event("input"))
// })
