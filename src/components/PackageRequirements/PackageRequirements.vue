<script setup lang="ts">
import type { Ref } from 'vue'

import DirectoryPathRequirement from './DirectoryPathRequirement.vue'

const { requirements } = defineProps<{ requirements: Map<string, string> }>()

const emit = defineEmits<{
  (event: 'requirementSatisfied', serialized: Map<string, string>): void
  (event: 'requirementNotSatisfied'): void
}>()

const satisfiedRequirements: Ref<Map<string, string | null>> = ref(new Map())

/* ======================== COMPONENT SATISFACTION ========================= */
class RequirementComponent {
  constructor(
    public comp: any,
    public prop: object = {},
    public even: object = {},
  ) {}
}

const requirementSatisfied = (requirementKey: string, serialized: string) => {
  satisfiedRequirements.value.set(requirementKey, serialized)
}

const requirementNotSatisfied = (requirementKey: string) => {
  satisfiedRequirements.value.set(requirementKey, null)
}

const componentFromString = (key: string, kind: string): RequirementComponent => {
  if (kind === 'DirectoryPath') {
    return new RequirementComponent(DirectoryPathRequirement, {}, {
      requirementSatisfied: (serialized: string) => requirementSatisfied(key, serialized),
      requirementNotSatisfied: () => requirementNotSatisfied(key),
    })
  }
  throw new Error(`Requirement of kind \`${kind}\` is not support`)
}

const requirementComponents: Ref<RequirementComponent[]> = shallowRef([])

const checkSatisfiedRequirements = (requirements: Map<string, string | null>) => {
  const satisfied: Map<string, string> = new Map()
  for (const [key, value] of requirements.entries()) {
    if (!value) {
      emit('requirementNotSatisfied')
      return
    }
    satisfied.set(key, value)
  }

  emit('requirementSatisfied', satisfied)
}
/* ======================== ========= ============ ========================= */

const replaceRequirements = (requirements: Map<string, string>) => {
  const requirementsArray = [...requirements.entries()]
  requirementComponents.value = requirementsArray.map(([key, kind]) => componentFromString(key, kind))
  satisfiedRequirements.value = new Map(requirementsArray.map(([key, _]) => [key, null]))
}

watch(satisfiedRequirements, checkSatisfiedRequirements, { deep: true })
watch(() => requirements, replaceRequirements)

onMounted(() => {
  replaceRequirements(requirements)
  checkSatisfiedRequirements(satisfiedRequirements.value)
})

const errorMessage = ref('')
onErrorCaptured((error) => {
  errorMessage.value = error.message
})
</script>

<template>
  <div
    h-full w-full
    flex flex-col flex-grow
    gap-4
    p-4
  >
    <div v-for="(requirement, index) in requirementComponents" :key="index">
      <!-- ERROR - SHOW ERROR WHEN CHILD COMPONENT ERROR MESSAGE CAUGHT -->
      <div v-if="errorMessage">
        <Error :message="errorMessage" />
      </div>
      <Suspense v-else>
        <!-- DONE - SHOW DYNAMIC COMPONENT -->
        <template #default>
          <component
            :is="requirement.comp"
            v-bind="requirement.prop"
            v-on="requirement.even"
          />
        </template>
        <!-- LOADING - SHOW LOADING ANIMATION -->
        <template #fallback>
          <Loading />
        </template>
      </Suspense>
    </div>
  </div>
</template>
