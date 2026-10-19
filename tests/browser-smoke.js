async () => {
  const canvas = document.querySelector('canvas');
  const held = new Set();
  const checks = [];
  const frames = async (count = 3) => {
    for (let i = 0; i < count; i++) await new Promise(requestAnimationFrame);
  };
  const seconds = async duration => {
    const end = performance.now() + duration * 1000;
    while (performance.now() < end) await frames();
  };
  const state = () => {
    const values = {...document.querySelector('#game-status').dataset};
    for (const name of ['x','y','z','yaw','yawRate','vx','vy','vz','speed','hull','energy','heat','delivered','cargo','tether','elapsed','fps','threat','downed','shots','incoming']) {
      values[name] = Number(values[name]);
    }
    return values;
  };
  const key = (code, down) => {
    if (held.has(code) === down) return;
    if (down) held.add(code); else held.delete(code);
    const symbol = ({Space:' ',ControlLeft:'Control',ShiftLeft:'Shift',Enter:'Enter',Escape:'Escape',F2:'F2'})[code] ?? code.replace('Key','').toLowerCase();
    canvas.dispatchEvent(new KeyboardEvent(down ? 'keydown' : 'keyup', {code,key:symbol,bubbles:true}));
  };
  const release = () => { for (const code of [...held]) key(code,false); };
  const tap = async code => { key(code,true); await frames(); key(code,false); await frames(); };
  const check = (condition, message) => { if (!condition) throw new Error(message); checks.push(message); };
  const basic = async () => {
    canvas.focus();
    check(state().phase === 'Intro', 'intro ready');
    await tap('Enter');
    key('KeyS',true);
    await frames(12);
    check(state().phase === 'Flying', 'start flight');
    await tap('KeyE');
    check(state().cargo === -1, 'remote pickup rejected');
    key('KeyA',true); await seconds(1.2); key('KeyA',false); await seconds(0.2);
    check(state().yaw > 0.4, 'A turns left');
    key('KeyD',true); await seconds(1.8); key('KeyD',false); await seconds(0.4);
    check(state().yaw < 0.2, 'D turns right');
    const altitude = state().y;
    key('Space',true); await seconds(3); key('Space',false); await seconds(0.2);
    check(state().y > altitude + 3, 'climb thrust');
    await tap('KeyF'); await frames(12);
    check(state().shots > 0, 'pulse weapon fires');
    key('ControlLeft',true); await frames(30);
    check(state().shield === 'true', 'shield activates');
    key('ControlLeft',false);
    release();
    await tap('KeyH'); await frames(10);
    const paused = state();
    await frames(30);
    check(paused.phase === 'Paused' && state().elapsed === paused.elapsed, 'help freezes simulation');
    await tap('KeyH'); await frames(10);
    check(state().phase === 'Flying', 'help resumes flight');
    await tap('Escape'); await frames(10);
    check(state().phase === 'Paused', 'escape pauses');
    await tap('KeyR'); await frames(10);
    check(state().phase === 'Intro' && state().delivered === 0 && state().elapsed === 0, 'restart clears expedition');
    return {checks,state:state()};
  };
  const relics = [[-87,50,1],[90,76,-129],[-22,110,-331]];
  const islands = [[0,15,156,47],[-98,24,-4,49],[97,49,-140,59],[-22,78,-339,74],[134,-1,68,23],[-195,65,-171,32],[220,99,-305,42],[-270,12,60,48]];
  let lastE = -5;
  const drive = async (milliseconds = 18000) => {
    canvas.focus();
    if (state().phase === 'Intro' || state().phase === 'Paused') await tap('Enter');
    const started = performance.now();
    while (performance.now() - started < milliseconds) {
      const s = state();
      if (s.phase === 'Won') { release(); return {checks,state:s}; }
      if (s.phase !== 'Flying') { release(); throw new Error(`Unexpected phase ${s.phase}: ${s.notice}`); }
      const target = s.cargo >= 0 ? [0,38,138] : relics[s.delivered];
      const dx=target[0]-s.x, dy=target[1]-s.y, dz=target[2]-s.z;
      const horizontal = Math.hypot(dx,dz);
      const distance = Math.hypot(dx,dy,dz);
      const desired = Math.atan2(-dx,-dz);
      const error = Math.atan2(Math.sin(desired-s.yaw),Math.cos(desired-s.yaw));
      let altitude=target[1];
      for (const [x,y,z,radius] of islands) {
        if (Math.hypot(s.x-x,s.z-z) < radius+20) altitude=Math.max(altitude,y+20+(s.cargo>=0?8:0));
      }
      if (s.threat>0 && horizontal>70) altitude+=Math.sin(s.elapsed*1.8)*8;
      const turn=Math.max(-1,Math.min(1,error*1.7-s.yawRate*0.55));
      const climb=Math.max(-1,Math.min(1,((altitude-s.y)*0.65-s.vy*0.8)/6));
      const leavingHarbor=s.cargo<0 && s.delivered>0 && Math.hypot(s.x,s.z-138)<55 && s.y<65;
      const holdForHeight=Math.abs(dy)>12 && horizontal<Math.abs(dy)*3;
      const pulse=(s.elapsed*7)%1;
      key('KeyA',turn>0 && pulse<turn);
      key('KeyD',turn<0 && pulse < -turn);
      key('Space',climb>0 && pulse<climb);
      key('KeyC',climb<0 && pulse < -climb);
      key('KeyS',(horizontal<50 || Math.abs(error)>0.85 || leavingHarbor || holdForHeight) && s.speed>3);
      key('KeyW',horizontal>55 && Math.abs(error)<0.65 && !leavingHarbor && !holdForHeight);
      key('ShiftLeft',s.threat>0 && Math.abs(error)<0.4 && horizontal>80 && s.energy>50 && s.heat<75 && !leavingHarbor && !holdForHeight);
      key('ControlLeft',s.incoming<50 && s.energy>3);
      if (distance<(s.cargo>=0?21:18) && s.speed<17 && s.elapsed-lastE>1) {
        const previous=`${s.delivered}:${s.cargo}`;
        await tap('KeyE'); await frames(8);
        lastE=s.elapsed;
        const next=state();
        if (`${next.delivered}:${next.cargo}` !== previous) checks.push(`interaction ${previous} -> ${next.delivered}:${next.cargo}`);
      }
      await frames(2);
    }
    release();
    await tap('Escape'); await frames(8);
    return {checks,state:state()};
  };
  window.stormQA = {state,key,tap,release,frames,basic,drive,checks};
  return state();
}
