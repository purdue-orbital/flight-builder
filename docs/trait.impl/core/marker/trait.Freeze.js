(function() {
    var implementors = Object.fromEntries([["flight_builder",[["impl !<a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/scheduler/struct.Scheduler.html\" title=\"struct flight_builder::scheduler::Scheduler\">Scheduler</a>",1,["flight_builder::scheduler::Scheduler"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/events/struct.RegisteredEvent.html\" title=\"struct flight_builder::events::RegisteredEvent\">RegisteredEvent</a>",1,["flight_builder::events::RegisteredEvent"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/scheduler/struct.PerATick.html\" title=\"struct flight_builder::scheduler::PerATick\">PerATick</a>",1,["flight_builder::scheduler::PerATick"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/scheduler/struct.Startup.html\" title=\"struct flight_builder::scheduler::Startup\">Startup</a>",1,["flight_builder::scheduler::Startup"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/scheduler/struct.Update.html\" title=\"struct flight_builder::scheduler::Update\">Update</a>",1,["flight_builder::scheduler::Update"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/states/struct.RegisteredState.html\" title=\"struct flight_builder::states::RegisteredState\">RegisteredState</a>",1,["flight_builder::states::RegisteredState"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/states/struct.RegisteredTransition.html\" title=\"struct flight_builder::states::RegisteredTransition\">RegisteredTransition</a>",1,["flight_builder::states::RegisteredTransition"]],["impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/query/struct.Res.html\" title=\"struct flight_builder::query::Res\">Res</a>&lt;'a, T&gt;",1,["flight_builder::query::Res"]],["impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/query/struct.ResMut.html\" title=\"struct flight_builder::query::ResMut\">ResMut</a>&lt;'a, T&gt;",1,["flight_builder::query::ResMut"]],["impl&lt;Input, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/tasks/struct.FunctionTask.html\" title=\"struct flight_builder::tasks::FunctionTask\">FunctionTask</a>&lt;Input, F&gt;<div class=\"where\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,</div>",1,["flight_builder::tasks::FunctionTask"]],["impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"enum\" href=\"flight_builder/states/enum.NextState.html\" title=\"enum flight_builder::states::NextState\">NextState</a>&lt;S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,</div>",1,["flight_builder::states::NextState"]],["impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/events/struct.EventReader.html\" title=\"struct flight_builder::events::EventReader\">EventReader</a>&lt;S&gt;",1,["flight_builder::events::EventReader"]],["impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/events/struct.EventWriter.html\" title=\"struct flight_builder::events::EventWriter\">EventWriter</a>&lt;S&gt;",1,["flight_builder::events::EventWriter"]],["impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/states/struct.OnEnter.html\" title=\"struct flight_builder::states::OnEnter\">OnEnter</a>&lt;S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,</div>",1,["flight_builder::states::OnEnter"]],["impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/states/struct.OnExit.html\" title=\"struct flight_builder::states::OnExit\">OnExit</a>&lt;S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,</div>",1,["flight_builder::states::OnExit"]],["impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/states/struct.OnTransition.html\" title=\"struct flight_builder::states::OnTransition\">OnTransition</a>&lt;S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,</div>",1,["flight_builder::states::OnTransition"]],["impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/states/struct.State.html\" title=\"struct flight_builder::states::State\">State</a>&lt;S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,</div>",1,["flight_builder::states::State"]],["impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/states/struct.StateTransitionEvent.html\" title=\"struct flight_builder::states::StateTransitionEvent\">StateTransitionEvent</a>&lt;S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,</div>",1,["flight_builder::states::StateTransitionEvent"]],["impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/states/struct.Transition.html\" title=\"struct flight_builder::states::Transition\">Transition</a>&lt;S&gt;",1,["flight_builder::states::Transition"]],["impl&lt;const CLOCK: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.86.0/std/primitive.u32.html\">u32</a>&gt; !<a class=\"trait\" href=\"https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"flight_builder/scheduler/struct.TaskRunner.html\" title=\"struct flight_builder::scheduler::TaskRunner\">TaskRunner</a>&lt;CLOCK&gt;",1,["flight_builder::scheduler::TaskRunner"]]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[8375]}