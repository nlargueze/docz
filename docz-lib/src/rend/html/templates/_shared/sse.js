window.addEventListener('load', () => {
    const evtSource = new EventSource("/ss-events")
    console.log('SSE (created)', evtSource)

    evtSource.onopen = (event) => {
        console.log('SSE (open)', event)
    }

    evtSource.onmessage = (event) => {
        console.log('SSE (message)', event)
        evtSource.close()
        window.location.reload()
    };

    evtSource.onerror = (err) => {
        console.error("SSE (error)", err)
    };

})
