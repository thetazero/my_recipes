import index from './index.js'

function query(list, search) {
    return list.filter(item => item.name.toLowerCase().includes(search.toLowerCase()))
}

console.log(index)

const searchElem = document.querySelector('#search')
const searchResultsElem = document.querySelector('#search-results')

searchElem.addEventListener('focus', () => {
    searchResultsElem.style.display = 'block'
    update_search()
})

searchElem.addEventListener('blur', () => {
    setTimeout(() => {
        searchResultsElem.style.display = 'none'
        clear_search()
    }, 1000 / 15)
})

searchElem.addEventListener('keyup', e => {
    console.log(e)
    if (e.key === 'Escape') {
        searchElem.blur()
    } else if (e.key === 'Enter') {
        const first_link = searchResultsElem.querySelector('a')
        if (first_link) {
            first_link.click()
        }
    } else {
        update_search()
    }
})

function update_search() {
    const search_query = searchElem.value
    console.log('update', search_query)
    let results = query(index, search_query)
    results = results.slice(0, 8)
    searchResultsElem.innerHTML = results.map(item => `<li><a href="${item.path}">${item.name}</a></li>`).join('')
}

function clear_search() {
    searchElem.value = ''
    searchResultsElem.innerHTML = ''
}

clear_search()