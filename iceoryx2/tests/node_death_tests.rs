// Copyright (c) 2024 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache Software License 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
// which is available at https://opensource.org/licenses/MIT.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[generic_tests::define]
mod node_death_tests {
    use iceoryx2::config::Config;
    use iceoryx2::node::testing::__internal_node_staged_death;
    use iceoryx2::node::{NodeState, NodeView};
    use iceoryx2::prelude::*;
    use iceoryx2::service::Service;
    use iceoryx2_bb_posix::unique_system_id::UniqueSystemId;
    use iceoryx2_bb_testing::assert_that;

    trait Test {
        type Service: Service;

        fn generate_node_name(i: usize, prefix: &str) -> NodeName {
            NodeName::new(&(prefix.to_string() + &i.to_string())).unwrap()
        }

        fn staged_death(node: &mut Node<Self::Service>);
    }

    struct ZeroCopy;

    impl Test for ZeroCopy {
        type Service = iceoryx2::service::zero_copy::Service;

        fn staged_death(node: &mut Node<Self::Service>) {
            use iceoryx2_cal::monitoring::testing::__InternalMonitoringTokenTestable;
            let monitor = unsafe { __internal_node_staged_death(node) };
            monitor.staged_death();
        }
    }

    #[test]
    fn dead_node_is_marked_as_dead_and_can_be_cleaned_up<S: Test>() {
        let node_name = S::generate_node_name(0, "toby or no toby");
        let fake_node_id = (u32::MAX as u128) << 96;
        let fake_node_id = unsafe { core::mem::transmute::<u128, UniqueSystemId>(fake_node_id) };

        let mut sut = unsafe {
            NodeBuilder::new()
                .name(node_name.clone())
                .__internal_create_with_custom_node_id::<S::Service>(fake_node_id)
                .unwrap()
        };

        S::staged_death(&mut sut);

        let mut node_list = vec![];
        Node::<S::Service>::list(Config::global_config(), |node_state| {
            node_list.push(node_state);
            CallbackProgression::Continue
        })
        .unwrap();
        assert_that!(node_list, len 1);

        if let Some(NodeState::Dead(state)) = node_list.pop() {
            assert_that!(*state.details().as_ref().unwrap().name(), eq node_name);
            assert_that!(state.remove_stale_resources(), eq Ok(true));
        } else {
            assert_that!(true, eq false);
        }

        node_list.clear();
        Node::<S::Service>::list(Config::global_config(), |node_state| {
            node_list.push(node_state);
            CallbackProgression::Continue
        })
        .unwrap();
        assert_that!(node_list, len 0);
    }

    #[instantiate_tests(<ZeroCopy>)]
    mod zero_copy {}
}
