import{w as N,B as o}from"./button-DW2OhPK9.js";import{q as v,r as n,l as e,t as x}from"./chunk-SYFQ2XB5-DqPEAYc-.js";import{c as b,C as f,a as y,b as w,I as h,T as k,B as g,S,D as C,f as I}from"./mcp-servers-Dp9BX-tx.js";/**
 * @license lucide-react v0.471.2 - ISC
 *
 * This source code is licensed under the ISC license.
 * See the LICENSE file in the root directory of this source tree.
 */const $=[["path",{d:"m12 19-7-7 7-7",key:"1l729n"}],["path",{d:"M19 12H5",key:"x3x0zl"}]],u=b("ArrowLeft",$);function E(i){const t=i.command.split(" "),s=t[0],d=t.slice(1);return`goose://extension?${[`cmd=${encodeURIComponent(s)}`,...d.map(a=>`arg=${encodeURIComponent(a)}`),`description=${encodeURIComponent(i.description)}`,...i.environmentVariables.filter(a=>a.required).map(a=>`env=${encodeURIComponent(`${a.name}=${a.description}`)}`)].join("&")}`}const R=N(function(){const{id:t}=v(),[s,d]=n.useState(null);n.useState(!0);const[j,a]=n.useState(null),[B,m]=n.useState(!0);return n.useEffect(()=>{(async()=>{try{m(!0),a(null);const c=(await I()).find(p=>p.id===t);if(!c){a(`Server with ID "${t}" not found`);return}d(c)}catch(l){const c=l instanceof Error?l.message:"Unknown error";a(`Failed to load server: ${c}`),console.error("Error loading server:",l)}finally{m(!1)}})()},[t]),s?e.jsx("div",{className:"container mx-auto",children:e.jsxs("div",{className:"flex gap-8 max-w-5xl mx-auto",children:[e.jsx("div",{children:e.jsx(x,{to:"/",children:e.jsxs(o,{className:"flex items-center gap-2",children:[e.jsx(u,{className:"h-4 w-4"}),"Back"]})})}),e.jsxs(f,{className:"p-8 w-full",children:[e.jsx(y,{className:"flex items-center",children:e.jsx("div",{className:"flex items-center gap-2",children:e.jsx("h1",{className:"font-medium text-5xl text-textProminent detail-page-server-name",children:s.name})})}),e.jsxs(w,{className:"space-y-6",children:[e.jsx("div",{children:e.jsx("p",{className:"text-xl text-textSubtle",children:s.description})}),e.jsx("div",{children:e.jsx("p",{className:"text-md text-textSubtle",children:s.installation_notes})}),e.jsx("div",{className:"space-y-2",children:s.is_builtin?e.jsxs("div",{className:"flex items-center gap-2 text-sm dark:text-gray-300",children:[e.jsx(h,{className:"h-4 w-4"}),"Can be enabled in the goose settings page"]}):e.jsxs(e.Fragment,{children:[e.jsxs("div",{className:"flex items-center gap-2 text-textStandard",children:[e.jsx(k,{className:"h-4 w-4"}),e.jsx("h4",{className:"font-medium",children:"Command"})]}),e.jsx("code",{className:"block bg-gray-100 dark:bg-gray-900 p-2 rounded text-sm dark:text-gray-300",children:`goose session --with-extension "${s.command}"`})]})}),e.jsxs("div",{className:"space-y-4",children:[e.jsx("h2",{className:"text-lg font-medium dark:text-gray-300",children:"Environment Variables"}),s.environmentVariables.length>0?e.jsx("div",{className:"",children:s.environmentVariables.map(r=>e.jsxs("div",{className:"border-b border-borderSubtle pb-4 mb-4 last:border-0",children:[e.jsx("div",{className:"text-sm dark:text-gray-300",children:r.name}),e.jsx("div",{className:"text-gray-600 dark:text-gray-400 text-sm mt-1",children:r.description}),r.required&&e.jsx(g,{variant:"secondary",className:"mt-2",children:"Required"})]},r.name))}):e.jsxs("div",{className:"text-gray-600 dark:text-gray-400 text-sm flex items-center gap-2",children:[e.jsx(h,{className:"h-4 w-4"}),"No environment variables needed"]})]}),e.jsxs("div",{className:"flex items-center justify-between",children:[e.jsxs("div",{className:"flex items-center gap-2 text-sm text-gray-500 dark:text-gray-400",children:[e.jsx(S,{className:"h-4 w-4"}),e.jsxs("span",{children:[s.githubStars," on Github"]})]}),e.jsx("a",{href:E(s),target:"_blank",rel:"noopener noreferrer",className:"no-underline",children:s.is_builtin?e.jsx("div",{className:"inline-block",title:"This extension is built into goose and can be enabled in the settings page",children:e.jsx(g,{variant:"secondary",className:"ml-2 text-xs cursor-help",children:"Built-in"})}):e.jsxs(o,{size:"icon",variant:"link",className:"group/download flex items-center justify-center text-xs leading-[14px] text-textSubtle px-0 transition-all",title:"Install with Goose",children:[e.jsx("span",{children:"Install"}),e.jsx(C,{className:"h-4 w-4 ml-2 group-hover/download:text-[#FA5204]"})]})})]})]})]})]})}):e.jsxs("div",{className:"max-w-4xl mx-auto",children:[e.jsxs("div",{className:"flex items-center gap-2 mb-6",children:[e.jsx(x,{to:"/",children:e.jsxs(o,{className:"",children:[e.jsx(u,{className:"h-4 w-4"}),"Back"]})}),e.jsxs("div",{className:"text-sm text-gray-500 dark:text-gray-400",children:[e.jsx(x,{to:"/",className:"hover:text-accent dark:hover:text-accent/90",children:"Goose Extensions"})," ","/"]})]}),e.jsxs("div",{className:"animate-pulse",children:[e.jsx("div",{className:"h-8 w-48 bg-gray-200 dark:bg-gray-700 rounded mb-4"}),e.jsx("div",{className:"h-4 w-full bg-gray-200 dark:bg-gray-700 rounded mb-2"}),e.jsx("div",{className:"h-4 w-2/3 bg-gray-200 dark:bg-gray-700 rounded"})]})]})});export{R as default};
