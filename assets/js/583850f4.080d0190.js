"use strict";(self.webpackChunkcompositor_live=self.webpackChunkcompositor_live||[]).push([[222],{6679:(e,n,o)=>{o.r(n),o.d(n,{assets:()=>d,contentTitle:()=>s,default:()=>p,frontMatter:()=>i,metadata:()=>c,toc:()=>a});var t=o(5893),r=o(1151);const i={},s="Component",c={id:"concept/component",title:"Component",description:"A component is a basic block used to define how video streams are composed.",source:"@site/pages/concept/component.md",sourceDirName:"concept",slug:"/concept/component",permalink:"/docs/concept/component",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{},sidebar:"sidebar",previous:{title:"Node.js",permalink:"/docs/get-started/node"},next:{title:"Layouts",permalink:"/docs/concept/layouts"}},d={},a=[{value:"Layout Components",id:"layout-components",level:2},{value:"Non-layout components",id:"non-layout-components",level:2},{value:"Scene",id:"scene",level:2},{value:"Renderers",id:"renderers",level:3}];function l(e){const n={a:"a",admonition:"admonition",code:"code",h1:"h1",h2:"h2",h3:"h3",li:"li",p:"p",pre:"pre",ul:"ul",...(0,r.a)(),...e.components};return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(n.h1,{id:"component",children:"Component"}),"\n",(0,t.jsx)(n.p,{children:"A component is a basic block used to define how video streams are composed."}),"\n",(0,t.jsx)(n.h2,{id:"layout-components",children:"Layout Components"}),"\n",(0,t.jsx)(n.p,{children:"Layout component is a type of component responsible for defining the size and position of other components."}),"\n",(0,t.jsx)(n.p,{children:"Currently, we support the following layout components:"}),"\n",(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsx)(n.li,{children:(0,t.jsx)(n.a,{href:"../api/components/View",children:"View"})}),"\n",(0,t.jsx)(n.li,{children:(0,t.jsx)(n.a,{href:"../api/components/Tiles",children:"Tiles"})}),"\n",(0,t.jsx)(n.li,{children:(0,t.jsx)(n.a,{href:"../api/components/Rescaler",children:"Rescaler"})}),"\n"]}),"\n",(0,t.jsxs)(n.p,{children:["Learn more about layouts ",(0,t.jsx)(n.a,{href:"./layouts",children:"here"}),"."]}),"\n",(0,t.jsx)(n.h2,{id:"non-layout-components",children:"Non-layout components"}),"\n",(0,t.jsx)(n.p,{children:"Non-layout components have their unique behaviors. In most cases, they do not support or interact with mechanisms introduced by layouts. Sometimes, they even override the behavior of other components."}),"\n",(0,t.jsxs)(n.p,{children:["For example, if you create a ",(0,t.jsx)(n.code,{children:"Shader"})," component with a ",(0,t.jsx)(n.code,{children:"View"})," component as its child, the properties like ",(0,t.jsx)(n.code,{children:"width"}),", ",(0,t.jsx)(n.code,{children:"top"}),", ",(0,t.jsx)(n.code,{children:"rotation"})," ..., will be ignored. A ",(0,t.jsx)(n.code,{children:"Shader"})," component, when rendering, receives all its children as GPU textures. It will just execute whatever the user-provided shader source implements without applying any layout properties that component might have."]}),"\n",(0,t.jsx)(n.h2,{id:"scene",children:"Scene"}),"\n",(0,t.jsx)(n.p,{children:"Component tree that represents what will be rendered for a specific output."}),"\n",(0,t.jsx)(n.p,{children:"Example scene:"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-typescript",children:'{\n    "type": "view",\n    "background_color_rgba": "#0000FFFF"\n    "children": [\n        {\n            "type": "input-stream",\n            "input_id": "example_input_1",\n        }\n    ]\n}\n'})}),"\n",(0,t.jsxs)(n.p,{children:["In the example above, we define a scene where an input stream ",(0,t.jsx)(n.code,{children:"example_input_1"})," is rendered inside a ",(0,t.jsxs)(n.a,{href:"../api/components/View",children:[(0,t.jsx)(n.code,{children:"View"})," component"]}),". You can configure that scene for a specific output in the ",(0,t.jsxs)(n.a,{href:"../api/routes#register-output-stream",children:[(0,t.jsx)(n.code,{children:"RegisterOutputStream"})," request"]})," using ",(0,t.jsx)(n.code,{children:"initial_scene"})," field or in the ",(0,t.jsxs)(n.a,{href:"../api/routes#update-scene",children:[(0,t.jsx)(n.code,{children:"UpdateScene"})," request"]}),"."]}),"\n",(0,t.jsx)(n.admonition,{type:"note",children:(0,t.jsxs)(n.p,{children:["You need to register ",(0,t.jsx)(n.code,{children:'"example_input_1"'})," before using it in the scene definition."]})}),"\n",(0,t.jsx)(n.h3,{id:"renderers",children:"Renderers"}),"\n",(0,t.jsx)(n.p,{children:"Renderers are entities capable of producing frames (in some cases based on some provided input). The renderer could be a WGSL shader, web renderer instance, or an image. They are not directly part of the scene definition. Instead, components are using them as part of their internal implementation."}),"\n",(0,t.jsx)(n.p,{children:"For example:"}),"\n",(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsxs)(n.li,{children:[(0,t.jsxs)(n.a,{href:"../api/components/Shader",children:["The ",(0,t.jsx)(n.code,{children:"Shader"})," component"]})," has a field ",(0,t.jsx)(n.code,{children:"shader_id"})," that identifies a ",(0,t.jsxs)(n.a,{href:"../api/renderers/Shader",children:[(0,t.jsx)(n.code,{children:"Shader"})," renderer"]}),"."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsxs)(n.a,{href:"../api/components/Image",children:["The ",(0,t.jsx)(n.code,{children:"Image"})," component"]})," has a field ",(0,t.jsx)(n.code,{children:"image_id"})," that identifies an ",(0,t.jsxs)(n.a,{href:"../api/renderers/Image",children:[(0,t.jsx)(n.code,{children:"Image"})," renderer"]}),"."]}),"\n"]}),"\n",(0,t.jsxs)(n.p,{children:["Every renderer, except ",(0,t.jsx)(n.a,{href:"../api/renderers/web",children:(0,t.jsx)(n.code,{children:"WebRenderer"})}),", can be used in multiple components. For example, you can create a single ",(0,t.jsx)(n.code,{children:"Shader"})," renderer that applies some effect and use that ",(0,t.jsx)(n.code,{children:"shader_id"})," in multiple ",(0,t.jsx)(n.code,{children:"Shader"})," components."]})]})}function p(e={}){const{wrapper:n}={...(0,r.a)(),...e.components};return n?(0,t.jsx)(n,{...e,children:(0,t.jsx)(l,{...e})}):l(e)}},1151:(e,n,o)=>{o.d(n,{Z:()=>c,a:()=>s});var t=o(7294);const r={},i=t.createContext(r);function s(e){const n=t.useContext(i);return t.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function c(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:s(e.components),t.createElement(i.Provider,{value:n},e.children)}}}]);