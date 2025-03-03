<!-- 
    This file is part of BestCraft.
    Copyright (C) 2024  Tnze

    BestCraft is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    BestCraft is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
-->

<script setup lang="ts">
import { ElForm, ElFormItem, ElSwitch, ElInputNumber } from 'element-plus';
import useGearsets from '@/stores/gearsets';
import { Jobs } from '@/libs/Craft';
import { computed } from 'vue';

const store = useGearsets();
const props = defineProps<{
    job: Jobs;
}>();

const v = computed(() => store.special.find(v => v.name == props.job)!);

function setInheritFromDefault(val: string | number | boolean) {
    // const v = store.special.find(v => v.name == props.job)!;
    if (val) {
        v.value.value = undefined;
    } else {
        v.value.value = { ...store.default };
    }
}

const displayValue = computed(() => v.value.value || store.default);
const isDefault = computed(() => v.value.value == null);
</script>

<template>
    <el-form v-if="v != undefined" label-position="right" label-width="auto">
        <el-form-item :label="$t('job')">
            {{ $t(String(job)) }}
        </el-form-item>
        <el-form-item :label="$t('attributes')">
            <el-switch
                :model-value="v.value == null"
                :active-text="$t('inherit-from-default')"
                @change="setInheritFromDefault"
            />
        </el-form-item>
        <el-form-item :label="$t('level')">
            <el-input-number
                :model-value="displayValue.level"
                :disabled="isDefault"
                :controls="!isDefault"
                :min="1"
                :max="100"
                :step-strictly="true"
                :value-on-clear="0"
                @change="x => (v.value!.level = x!)"
            />
        </el-form-item>
        <el-form-item :label="$t('craftsmanship')">
            <el-input-number
                :model-value="displayValue.craftsmanship"
                :disabled="isDefault"
                :controls="!isDefault"
                :min="0"
                :step-strictly="true"
                :value-on-clear="0"
                @change="x => (v.value!.craftsmanship = x!)"
            />
        </el-form-item>
        <el-form-item :label="$t('control')">
            <el-input-number
                :model-value="displayValue.control"
                :disabled="isDefault"
                :controls="!isDefault"
                :min="0"
                :step-strictly="true"
                :value-on-clear="0"
                @change="x => (v.value!.control = x!)"
            />
        </el-form-item>
        <el-form-item :label="$t('craft-point')">
            <el-input-number
                :model-value="displayValue.craft_points"
                :disabled="isDefault"
                :controls="!isDefault"
                :min="0"
                :step-strictly="true"
                :value-on-clear="0"
                @change="x => (v.value!.craft_points = x!)"
            />
        </el-form-item>
    </el-form>
</template>

<fluent locale="zh-CN">
job = 职业
attributes = 装备属性
inherit-from-default = 继承自默认
</fluent>

<fluent locale="en-US">
job = Job
attributes = Crafter Attributes
inherit-from-default = Inherit from default
</fluent>

<fluent locale="ja-JP">
attributes = 属性
inherit-from-default = デフォルトから継承
</fluent>
