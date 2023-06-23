
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr,insertEmpty } from 'blve/dist/runtime'
export default function(elm) {
    const refs = [0, false, null];
    let boolVal1 = reactiveValue(false, 1, refs)
    let boolVal2 = reactiveValue(false, 2, refs)
    let boolVal3 = reactiveValue(false, 4, refs)
    let boolVal4 = reactiveValue(false, 8, refs)
    let boolVal5 = reactiveValue(false, 16, refs)
    let boolVal6 = reactiveValue(false, 32, refs)

    elm.innerHTML = `<div id="_V_cXkudJBCOFPoLhJeoY"><div id="xrFgajvkxEXwyxooyHQwO">AAA</div></div>`;

    const [_V_cXkudJBCOFPoLhJeoYRef,xrFgajvkxEXwyxooyHQwORef] = getElmRefs(["_V_cXkudJBCOFPoLhJeoY","xrFgajvkxEXwyxooyHQwO"], 3);

    const TjqmjzHKMON_xpHsk$MOFAnchor = insertEmpty(_V_cXkudJBCOFPoLhJeoYRef,xrFgajvkxEXwyxooyHQwORef);

    const BvUTwTUUCOzgEQOQGINYXAnchor = insertEmpty(_V_cXkudJBCOFPoLhJeoYRef,xrFgajvkxEXwyxooyHQwORef);

    const EU$ACyJdPgqXwgTtZFQdmAnchor = insertEmpty(_V_cXkudJBCOFPoLhJeoYRef,null);

    const bmDvolUlQvqeLETyLNwHtAnchor = insertEmpty(_V_cXkudJBCOFPoLhJeoYRef,null);

    const renderTjqmjzHKMON_xpHsk$MOFElm = () => {
        let TjqmjzHKMON_xpHsk$MOFElm = document.createElement("div");
    
        TjqmjzHKMON_xpHsk$MOFElm.innerHTML = "AAA";
    
        _V_cXkudJBCOFPoLhJeoYRef.insertBefore(TjqmjzHKMON_xpHsk$MOFElm, TjqmjzHKMON_xpHsk$MOFAnchor);
    
    }

    boolVal1.v && renderTjqmjzHKMON_xpHsk$MOFElm()

    const renderBvUTwTUUCOzgEQOQGINYXElm = () => {
        let BvUTwTUUCOzgEQOQGINYXElm = document.createElement("div");
    
        BvUTwTUUCOzgEQOQGINYXElm.innerHTML = "AAA";
    
        _V_cXkudJBCOFPoLhJeoYRef.insertBefore(BvUTwTUUCOzgEQOQGINYXElm, BvUTwTUUCOzgEQOQGINYXAnchor);
    
    }

    boolVal2.v && renderBvUTwTUUCOzgEQOQGINYXElm()

    const renderYpNYCapwGYaTTITocToplElm = () => {
        let YpNYCapwGYaTTITocToplElm = document.createElement("div");
    
        YpNYCapwGYaTTITocToplElm.innerHTML = "AAA";
    
        _V_cXkudJBCOFPoLhJeoYRef.insertBefore(YpNYCapwGYaTTITocToplElm, xrFgajvkxEXwyxooyHQwORef);
    
    }

    boolVal3.v && renderYpNYCapwGYaTTITocToplElm()

    const renderEU$ACyJdPgqXwgTtZFQdmElm = () => {
        let EU$ACyJdPgqXwgTtZFQdmElm = document.createElement("div");
    
        EU$ACyJdPgqXwgTtZFQdmElm.innerHTML = "AAA";
    
        _V_cXkudJBCOFPoLhJeoYRef.insertBefore(EU$ACyJdPgqXwgTtZFQdmElm, EU$ACyJdPgqXwgTtZFQdmAnchor);
    
    }

    boolVal4.v && renderEU$ACyJdPgqXwgTtZFQdmElm()

    const renderbmDvolUlQvqeLETyLNwHtElm = () => {
        let bmDvolUlQvqeLETyLNwHtElm = document.createElement("div");
    
        bmDvolUlQvqeLETyLNwHtElm.innerHTML = "AAA";
    
        _V_cXkudJBCOFPoLhJeoYRef.insertBefore(bmDvolUlQvqeLETyLNwHtElm, bmDvolUlQvqeLETyLNwHtAnchor);
    
    }

    boolVal5.v && renderbmDvolUlQvqeLETyLNwHtElm()

    const renderSGjusaWNhjaSleIKIAMeOElm = () => {
        let SGjusaWNhjaSleIKIAMeOElm = document.createElement("div");
    
        SGjusaWNhjaSleIKIAMeOElm.innerHTML = "AAA";
    
        _V_cXkudJBCOFPoLhJeoYRef.insertBefore(SGjusaWNhjaSleIKIAMeOElm, null);
    
    }

    boolVal6.v && renderSGjusaWNhjaSleIKIAMeOElm()

    refs[2] = genUpdateFunc(() => {
    
    });

}