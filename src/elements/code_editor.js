
function waitForElement(selector) {
  return new Promise(resolve => {
    if (document.querySelector(selector)) {
      return resolve(document.querySelector(selector));
    }

    const observer = new MutationObserver((_mutations, observer) => {
      if (document.querySelector(selector)) {
        observer.disconnect();
        return resolve(document.querySelector(selector));
      }
    });

    observer.observe(
      document.body, 
      {
        childList: true,
        subtree: true
      }
    )
  })
}

const textArea = await waitForElement("#text-area-editor");
dioxus.send((textArea.selectionStart, textArea.selectionEnd))