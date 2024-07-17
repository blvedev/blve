import { BlveModuleExports, ComponentDeclaration } from ".";

export type ComponentLoader = () => Promise<{ default: ComponentDeclaration }>;

export type Route = {
  path: string;
  component: ComponentLoader;
};

export class Router {
  routes: Route[];
  notFound: () => void;
  currentComponent: BlveModuleExports | null;
  renderingTarget!: {
    parent: HTMLElement;
    anchor: HTMLElement | null;
    haveSiblingElm: boolean;
  };

  constructor() {
    this.routes = [];
    this.notFound = () => {};
    this.currentComponent = null;
    window.addEventListener("popstate", this.handlePopState.bind(this));
  }

  addRoute(path: string, componentLoader: ComponentLoader) {
    this.routes.push({ path, component: componentLoader });
  }

  setNotFound(notFoundHandler: () => void) {
    this.notFound = notFoundHandler;
  }

  navigate(path: string) {
    window.history.pushState({}, path, window.location.origin + path);
    this.handleRoute(path);
  }

  async handleRoute(path: string) {
    const route = this.routes.find((route) => route.path === path);
    if (route) {
      const component = (await route.component()).default;
      this.renderComponent(component);
    } else {
      this.notFound();
    }
  }

  handlePopState() {
    this.handleRoute(window.location.pathname);
  }

  renderComponent(component: ComponentDeclaration) {
    // TODO: Execute destroy function of the current component
    // if (this.currentComponent && this.currentComponent.__blve_destroy) {
    //   this.currentComponent.__blve_destroy();
    // }
    this.currentComponent = component();
    if (this.renderingTarget.haveSiblingElm) {
      this.currentComponent.insert(
        this.renderingTarget.parent,
        this.renderingTarget.anchor
      );
    } else {
      this.currentComponent.mount(this.renderingTarget.parent);
    }
  }

  initialize(
    routes: Route[] = [],
    parent: HTMLElement,
    anchor: HTMLElement | null,
    haveSiblingElm: boolean
  ) {
    this.routes = routes;
    this.renderingTarget = { parent, anchor, haveSiblingElm };
    this.handleRoute(window.location.pathname);
  }
}

export const $$blveRouter = new Router();
