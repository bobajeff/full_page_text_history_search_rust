(()=>{
    (function addPageBinding(type, bindingName) {
        /* Cast window to any here as we're about to add properties to it
         * via win[bindingName] which TypeScript doesn't like.
         */
        const win = window;
        const binding = win[bindingName];
        win[bindingName] = (...args) => {
            const me = window[bindingName];
            let callbacks = me.callbacks;
            if (!callbacks) {
                callbacks = new Map();
                me.callbacks = callbacks;
            }
            const seq = (me.lastSeq || 0) + 1;
            me.lastSeq = seq;
            const promise = new Promise((resolve, reject) => callbacks.set(seq, { resolve, reject }));
            binding(JSON.stringify({ type, name: bindingName, seq, args }));
            return promise;
        };
    })("exposedFun","addToText")

    function RunONDOMContentLoaded() {
        liveTextExtractor();//Need to wait for document.body to be available before running most of the operations
    };

    if (document.readyState === 'loading') {  // Loading hasn't finished yet
        document.addEventListener('DOMContentLoaded', RunONDOMContentLoaded);
    } else {  // `DOMContentLoaded` has already fired
        liveTextExtractor();
    }

    function removeListeners(){
        document.removeEventListener('DOMContentLoaded', RunONDOMContentLoaded);
    }
    //remove listeners after a minute has past
    //I want to keep them alive enough for the screenshot taker to have a chance at getting a decent shot
    setTimeout(removeListeners, 60000); // 1 minute
    

    
    function liveTextExtractor()
    {
        console.log('liveTextExtractor');//DEBUG
        var addedTextNodes = []; //Array for holding references to nodes that have been added
        function getTextFromDOMTree (node) {
            var textStrings = [];
            //Filter out 1) all the Script/NoScript/Style tags 2) any non-text nodes 3) any strings containing only whitespace characters 4) any non-visible text nodes 5) any nodes not added to the addedTextNodes array already
            var filter = {
                acceptNode: function(n) {
                    return n && n.parentNode && n.parentNode.tagName != "SCRIPT" && n.parentNode.tagName != "NOSCRIPT" && n.parentNode.tagName != "STYLE" && n.nodeType == Node.TEXT_NODE && /[^\s]/m.test(n.textContent) && (!!n.parentNode.clientHeight || !!n.parentNode.clientWidth || !!n.parentNode.getClientRects().length) && !addedTextNodes.includes(n)
                    ? NodeFilter.FILTER_ACCEPT
                    : NodeFilter.FILTER_REJECT;
                }
            };
            var nodes = document.createTreeWalker(node, NodeFilter.SHOW_TEXT, filter);
            var currentNode = nodes.currentNode;
            //run a check on the first node from the treewalker as it doesn't have to pass the filter
            if (currentNode != Node.TEXT_NODE)
            {
                currentNode = nodes.nextNode();
            }
            if (!currentNode)
            {
                //Exit out of function
                return textStrings;
            }
                while(currentNode) {
                    //Check to see if node was already added (still have to run this here because every loop I add more and the filter is only run on the ones added prior)
                    if (addedTextNodes.includes(currentNode))
                    {
                        //console.log('you already added this!');
                    } else {
                        var prefix = "";
                        if (!currentNode.previousSibling)
                        {
                            prefix = " ";//add a spcae if the node is the first child.
                        }
                        var textString = (prefix + currentNode.textContent).replace(/\s+/g, " ");//remove extra white space characters
                        textStrings.push(textString);
                        addedTextNodes.push(currentNode);
                        currentNode = nodes.nextNode();
                    }
                }
            return textStrings;
        }
        
        //Get text that's already loaded in the DOM and send to addToText function
        var initial_textStrings = getTextFromDOMTree(document.body);
        if (!!initial_textStrings.length){ //don't send back empty string
            console.log('addToText');
            window['addToText'](initial_textStrings);
        }
        
        //Observe changes to the DOM where text is added and send them to the addToText function
        const config = { childList: true, subtree: true, characterData: true, characterDataOldValue : true };
        const observer = new MutationObserver(function(mutations){
            var textStrings = [];
            mutations.forEach(function(mutation){
                if (mutation.type == 'childList')
                {
                    mutation.addedNodes.forEach((node)=>{
                        var new_textStrings = getTextFromDOMTree(node);
                        if (!!new_textStrings.length)
                        {
                            textStrings = textStrings.concat(new_textStrings);
                        }
                    });
                }
            })
            if (!!textStrings.length) //Don't add empty strings
            {
                console.log('addToText');
                window['addToText'](textStrings);
            }
        });
        observer.observe(document.body, config);
    }
})();
//# sourceURL=__puppeteer_evaluation_script__