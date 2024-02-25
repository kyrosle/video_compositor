"use strict";(self.webpackChunkcompositor_live=self.webpackChunkcompositor_live||[]).push([[7140],{6105:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>d,contentTitle:()=>o,default:()=>l,frontMatter:()=>a,metadata:()=>i,toc:()=>c});var s=n(5893),r=n(1151);const a={},o="Shaders",i={id:"concept/shaders",title:"Shaders",description:"Shaders are small programs that we send to a GPU to perform some computation for us. They are used extensively in the video compositor. All builtin transformation are implemented as shaders under the hood. It is also possible to create render nodes that run a custom shader on their input. Since video compositor is implemented using wgpu, the shaders have to be written in WGSL (WebGPU Shading Language). They also have to fulfill some custom requirements that allow them to be run by the video compositor.",source:"@site/pages/concept/shaders.md",sourceDirName:"concept",slug:"/concept/shaders",permalink:"/docs/concept/shaders",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{},sidebar:"sidebar",previous:{title:"Layouts",permalink:"/docs/concept/layouts"},next:{title:"Web Renderer",permalink:"/docs/concept/web"}},d={},c=[{value:"General concepts",id:"general-concepts",level:2},{value:"Vertex shaders",id:"vertex-shaders",level:3},{value:"Fragment shaders",id:"fragment-shaders",level:3},{value:"API",id:"api",level:2},{value:"Header",id:"header",level:3},{value:"Custom parameters",id:"custom-parameters",level:3},{value:"Entrypoints",id:"entrypoints",level:3}];function h(e){const t={a:"a",code:"code",h1:"h1",h2:"h2",h3:"h3",p:"p",pre:"pre",...(0,r.a)(),...e.components};return(0,s.jsxs)(s.Fragment,{children:[(0,s.jsx)(t.h1,{id:"shaders",children:"Shaders"}),"\n",(0,s.jsx)(t.p,{children:"Shaders are small programs that we send to a GPU to perform some computation for us. They are used extensively in the video compositor. All builtin transformation are implemented as shaders under the hood. It is also possible to create render nodes that run a custom shader on their input. Since video compositor is implemented using wgpu, the shaders have to be written in WGSL (WebGPU Shading Language). They also have to fulfill some custom requirements that allow them to be run by the video compositor."}),"\n",(0,s.jsx)(t.h2,{id:"general-concepts",children:"General concepts"}),"\n",(0,s.jsx)(t.p,{children:"There are two kinds of shaders that are used in the video compositor: vertex shaders and fragment shaders."}),"\n",(0,s.jsx)(t.h3,{id:"vertex-shaders",children:"Vertex shaders"}),"\n",(0,s.jsx)(t.p,{children:"Vertex shaders receive the data of a single vertex as input. It can manipulate them to make them form the shape we want to see as the output."}),"\n",(0,s.jsx)(t.p,{children:"The videos are represented in vertex shaders as two triangles, aligned like so:"}),"\n",(0,s.jsx)(t.pre,{children:(0,s.jsx)(t.code,{className:"language-console",children:" ______\n|     /|\n|    / |\n|   /  |\n|  /   |\n| /    |\n|______|\n"})}),"\n",(0,s.jsx)(t.p,{children:"The rectangle formed by these triangles spans the whole clip space, i.e. [-1, 1] X [-1, 1]."}),"\n",(0,s.jsxs)(t.p,{children:["Each video passed in as input gets a separate rectangle.\n",(0,s.jsx)(t.code,{children:"plane_id"})," in ",(0,s.jsx)(t.code,{children:"base_params"})," ",(0,s.jsx)(t.code,{children:"push_constant"})," represents number of currently rendered plane (texture)."]}),"\n",(0,s.jsxs)(t.p,{children:["If there are no input textures, ",(0,s.jsx)(t.code,{children:"plane_id"})," is equal to -1 and a single rectangle is passed to the shader. It is only useful for shaders that generate something in the fragment shader."]}),"\n",(0,s.jsxs)(t.p,{children:["Since the compositor doesn't deal with complex geometry and most positioning/resizing/cropping should be taken care of by ",(0,s.jsx)(t.a,{href:"https://compositor.live/docs/concept/layouts",children:"layouts"}),", we don't expect the users to write nontrivial vertex shaders very often. For just applying some effects to the video, fragment shaders are the way to go. This vertex shader should take care of most of your needs (for transformations that receive a single video and only process it in the fragment shader):"]}),"\n",(0,s.jsx)(t.pre,{children:(0,s.jsx)(t.code,{className:"language-wgsl",children:"struct VertexOutput {\n    @builtin(position) position: vec4<f32>,\n    @location(0) tex_coords: vec2<f32>,\n}\n\n@vertex\nfn vs_main(input: VertexInput) -> VertexOutput {\n    var output: VertexOutput;\n\n    output.position = vec4(input.position, 1.0);\n    output.tex_coords = input.tex_coords;\n\n    return output;\n}\n"})}),"\n",(0,s.jsx)(t.h3,{id:"fragment-shaders",children:"Fragment shaders"}),"\n",(0,s.jsxs)(t.p,{children:["A single instance of a fragment shader is started for each pixel of the output texture. That instance is responsible for calculating the color of the pixel. The return type of a fragment shader has to be a 4-element long vector of ",(0,s.jsx)(t.code,{children:"f32"}),"s ranging from 0.0 to 1.0. These floats are the RGBA values of the pixel."]}),"\n",(0,s.jsxs)(t.p,{children:["The fragment shader often receives texture coordinates, which describe where to sample the provided texture to get the color value corresponding to the pixel we're calculating at the moment. The texture can be sampled using the builtin ",(0,s.jsx)(t.code,{children:"textureSample"})," function. The sampler that should be used for sampling the texture is provided in the header is called ",(0,s.jsx)(t.code,{children:"sampler_"})]}),"\n",(0,s.jsx)(t.pre,{children:(0,s.jsx)(t.code,{className:"language-wgsl",children:"let color = textureSample(texture, sampler_, texture_coordinates)\n"})}),"\n",(0,s.jsx)(t.p,{children:"For example see this simple fragment shader, which applies the negative effect:"}),"\n",(0,s.jsx)(t.pre,{children:(0,s.jsx)(t.code,{className:"language-wgsl",children:"struct VertexOutput {\n    @builtin(position) position: vec4<f32>,\n    @location(0) tex_coords: vec2<f32>,\n}\n\n@fragment\nfn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {\n    let color = textureSample(textures[0], sampler_, input.tex_coords);\n    return vec4(vec3(1.0) - color, 1.0);\n}\n"})}),"\n",(0,s.jsx)(t.h2,{id:"api",children:"API"}),"\n",(0,s.jsx)(t.h3,{id:"header",children:"Header"}),"\n",(0,s.jsx)(t.p,{children:"Every user-provided shader should include the code below."}),"\n",(0,s.jsx)(t.pre,{children:(0,s.jsx)(t.code,{className:"language-wgsl",children:"struct VertexInput {\n    @location(0) position: vec3<f32>,\n    @location(1) tex_coords: vec2<f32>,\n}\n\nstruct BaseShaderParameters {\n    plane_id: i32,\n    time: f32,\n    output_resolution: vec2<u32>,\n    texture_count: u32,\n}\n\n@group(0) @binding(0) var textures: binding_array<texture_2d<f32>, 16>;\n@group(2) @binding(0) var sampler_: sampler;\n\nvar<push_constant> base_params: BaseShaderParameters;\n"})}),"\n",(0,s.jsx)(t.h3,{id:"custom-parameters",children:"Custom parameters"}),"\n",(0,s.jsx)(t.p,{children:"You can define a custom WGSL struct and bind a value of this type as"}),"\n",(0,s.jsx)(t.pre,{children:(0,s.jsx)(t.code,{className:"language-wgsl",children:"@group(1) @binding(0) var<uniform> custom_name: CustomStruct;\n"})}),"\n",(0,s.jsxs)(t.p,{children:["This struct has to be provided when creating a node using the ",(0,s.jsx)(t.code,{children:"shader_params"})," field of the ",(0,s.jsx)(t.a,{href:"https://github.com/membraneframework/video_compositor/wiki/API-%E2%80%90-nodes#shader",children:"shader node struct"})]}),"\n",(0,s.jsx)(t.h3,{id:"entrypoints",children:"Entrypoints"}),"\n",(0,s.jsx)(t.p,{children:"The vertex shader entrypoint has to have the following signature:"}),"\n",(0,s.jsx)(t.pre,{children:(0,s.jsx)(t.code,{className:"language-wgsl",children:"@vertex\nfn vs_main(input: VertexInput) -> A\n"})}),"\n",(0,s.jsxs)(t.p,{children:["Where ",(0,s.jsx)(t.code,{children:"A"})," can be any user-defined struct suitable for a vertex shader output."]}),"\n",(0,s.jsx)(t.p,{children:"The fragment shader entrypoint has to have the following signature:"}),"\n",(0,s.jsx)(t.pre,{children:(0,s.jsx)(t.code,{className:"language-wgsl",children:"@fragment\nfn fs_main(input: A) -> @location(0) vec4<f32>\n"})}),"\n",(0,s.jsxs)(t.p,{children:["Where ",(0,s.jsx)(t.code,{children:"A"})," is the output type of the vertex shader."]}),"\n",(0,s.jsxs)(t.p,{children:["Shaders have to be registered using the ",(0,s.jsx)(t.a,{href:"https://github.com/membraneframework/video_compositor/wiki/Api-%E2%80%90-renderers#shader",children:"register shader"})," request before they can be used."]})]})}function l(e={}){const{wrapper:t}={...(0,r.a)(),...e.components};return t?(0,s.jsx)(t,{...e,children:(0,s.jsx)(h,{...e})}):h(e)}},1151:(e,t,n)=>{n.d(t,{Z:()=>i,a:()=>o});var s=n(7294);const r={},a=s.createContext(r);function o(e){const t=s.useContext(a);return s.useMemo((function(){return"function"==typeof e?e(t):{...t,...e}}),[t,e])}function i(e){let t;return t=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:o(e.components),s.createElement(a.Provider,{value:t},e.children)}}}]);