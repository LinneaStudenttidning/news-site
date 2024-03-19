// TODO: This dependency is not listed anywhere else.
// Keep this TODO as a remember not for updating it if need be.
// FIXME: This could also be self hosted.
import * as marked from "https://cdn.jsdelivr.net/npm/marked@12.0.1/lib/marked.esm.js"

const renderer = {
    paragraph: text => `<span class="p">${text}</span>`,
    heading: (text, level) => {
        const tags = new Array(level).fill("#").join(" ")
        return `<span class="heading"><span class="special-char">${tags} </span>${text}</span>`
    },
    strong: text => {
        const asterisks = `<span class="special-char">**</span>`
        return `<strong>${asterisks}${text}${asterisks}</strong>`
    },
    em: text => {
        const asterisks = `<span class="special-char">*</span>`
        return `<em>${asterisks}${text}${asterisks}</em>`
    },
    list: (body, ordered, start) => {
        const listType = ordered ? "ol" : "ul"
        const startAt = start ? `start="${start}"` : ""
        return `<span class="${listType}" ${startAt}>${body}</span>`
    },
    listitem: (text, task, checked) => {
        return `<span class="list-item">${text}</span><br>`
    }
}

marked.use({
    gfm: true,
    breaks: true,
    renderer
})

const editors = document.querySelectorAll(".editor")

editors.forEach(editor => {
    const preview = editor.querySelector("pre")
    const textbox = editor.querySelector(".textbox")

    textbox.addEventListener("input", () => {
        const md = textbox.innerText.replace(/\n\n/g, "\n \n")
        const html = marked.parse(md)
        preview.innerHTML = html

        console.log(html)
        console.log(md)
    })

    textbox.addEventListener("scroll", () => preview.scrollTop = textbox.scrollTop)
})