export type SymbolRefInfo = [number, boolean, (() => void) | null];

export class valueObj<T> {
	constructor(
		private _v: T,
		private symbolIndex: number,
		readonly symbolRef: SymbolRefInfo,
	) {}

	set v(v: T) {
		if (this._v === v) return;
		this._v = v;
		this.symbolRef[0] = this.symbolIndex | this.symbolRef[0];
		if (!this.symbolRef[1]) {
			Promise.resolve().then(this.symbolRef[2]?.bind(this));
			this.symbolRef[1] = true;
		}
	}

	get v() {
		return this._v;
	}
}

export function genUpdateFunc(updElm: () => void) {
	return function (this: valueObj<any>) {
		if (this.symbolRef[1]) {
			updElm();
			this.symbolRef[0] = 0;
			this.symbolRef[1] = false;
		}
	};
}

export function escapeHtml(text: any): string {
	const map: { [key: string]: string } = {
		"&": "&amp;",
		"<": "&lt;",
		">": "&gt;",
		'"': "&quot;",
		"'": "&#039;",
	};

	return String(text).replace(/[&<>"']/g, function (m: string): string {
		return map[m];
	});
}

export function replaceText(content: any, elm: HTMLElement) {
	elm.innerHTML = escapeHtml(content);
}

export function replaceAttr(key: string, content: any, elm: HTMLElement) {
	if (content === undefined && elm.hasAttribute(key)) {
		elm.removeAttribute(key);
		return;
	}
	(elm as any)[key] = String(content);
}

export function reactiveValue<T>(
	v: T,
	symbolIndex: number,
	symbolRef: SymbolRefInfo,
) {
	return new valueObj<T>(v, symbolIndex, symbolRef);
}

export function addEvListener(
	elm: HTMLElement,
	evName: string,
	evFunc: EventListener,
) {
	elm.addEventListener(evName, evFunc);
}

export function getElmRefs(ids: string[], preserveId: number) {
	return ids.map((id, index) => {
		const e = document.getElementById(id)!;
		(2 ** index) & preserveId && e.removeAttribute("id");
		return e;
	});
}

export function insertEmpty(parent: HTMLElement, anchor: HTMLElement|null) {
	const empty = document.createTextNode(" ");
	parent.insertBefore(empty, anchor);
	return empty;
}
